#define SMALLEST_DIST (float)0.01
#define NORMAL_EPSILON (float)0.1
#define MAX_ITERATIONS 100
#define MAX_DIST 100
#define MAX_REFLECTION_DEPTH 3
#define MIN_REFLECTION_CUTOFF 0.05

#define SCALE_UCHAR3_BY_FLOAT(a, f) (uchar3)((uchar)((float)a.s0*f), (uchar)((float)a.s1*f), (uchar)((float)a.s2*f))

#define OBJECT_TYPE(a) a.s0
#define OBJECT_COLOR(a) a.s123

#define CAMERA_POS(a) a.s012
#define CAMERA_ROTATION(a) a.s345
#define CAMERA_FRAME_DIST(a) a.s6
#define CAMERA_SCALE(a) a.s7

#define REFLECTIVITY(a) a.sF

#define SPHERE 0
#define SPHERE_POS(a) a.s012
#define SPHERE_RADIUS(a) a.s3

#define FLOORPLANE 1
#define FLOORPLANE_POS(a) a.s0

#define CAPSULE 2
#define CAPSULE_POS_1(a) a.s012
#define CAPSULE_POS_2(a) a.s345
#define CAPSULE_RADIUS(a) a.s6

#define CYLINDER 3
#define CYLINDER_POS_1(a) a.s012
#define CYLINDER_POS_2(a) a.s345
#define CYLINDER_RADIUS(a) a.s6

#define BOX 4
#define BOX_POS(a) a.s012
#define BOX_SCALING(a) a.s345
#define BOX_ROTATION(a) a.s678

struct ClosePoint {
  float3 point;
  uint iterations;
  uint obj_index;
  bool out_of_bounds;
};

struct SceneDist {
  float dist;
  uint obj_index;
};

float sphereDist(float16 sphere_data, float3 point) {
  float3 pos = SPHERE_POS(sphere_data);
  return fast_distance(point, pos) - SPHERE_RADIUS(sphere_data);
}

float floorplaneDist(float16 floor_data, float3 point) {
  return point.s1 - FLOORPLANE_POS(floor_data);
}

float capsuleDist(float16 capsule_data, float3 point) {
  float3 a = CAPSULE_POS_1(capsule_data);
  float3 b = CAPSULE_POS_2(capsule_data);

  float3 ab = b-a;
  float3 ap = point-a;
  float t = dot(ab, ap) / dot(ab, ab);

  t = clamp(t, (float)0, (float)1);

  float3 proj = a + t*ab;
    
  return fast_distance(point, proj) - CAPSULE_RADIUS(capsule_data);
}

float cylinderDist(float16 cylinder_data, float3 point) {
  float3 a = CYLINDER_POS_1(cylinder_data);
  float3 b = CYLINDER_POS_2(cylinder_data);

	float3 ab = b-a;
  float3 ap = point-a;
  float t = dot(ab, ap) / dot(ab, ab);
  
  float3 proj = a + t*ab;
  
  float x = fast_distance(point,proj)-CYLINDER_RADIUS(cylinder_data);
  float y = (fabs(t-.5)-.5)*fast_length(ab);
  float e = fast_length(fmax((float2)(x,y), (float)0));
  float i = fmin(fmax(x, y), (float)0);
  
  return e+i;
}

float3 vecRotate(float3 pos,float3 rotation) {
  float cosa = cos(rotation.s0);
  float sina = sin(rotation.s0);

  float cosb = cos(rotation.s1);
  float sinb = sin(rotation.s1);

  float cosc = cos(rotation.s2);
  float sinc = sin(rotation.s2);

  float x = dot((float3)(cosc*cosb, cosc*sinb*sina - sinc*cosa, cosc*sinb*cosa + sinc*sina), pos);
  float y = dot((float3)(sinc*cosb, sinc*sinb*sina + cosc*cosa, sinc*sinb*cosa - cosc*sina), pos);
  float z = dot((float3)(-sinb    , cosb*sina                 , cosb*cosa                 ), pos);

  return (float3)(x,y,z);
}

float3 vecRotateAround(float3 pos,float3 rotation, float3 around) {
  return vecRotate(pos - around, rotation) + around;
}

float boxDist( float16 box_data, float3 point)
{
  float3 rot = BOX_ROTATION(box_data);
  float3 scale = BOX_SCALING(box_data);
  float3 pos = BOX_POS(box_data);

  float3 tpos = vecRotateAround(point, rot, pos + scale/2) - pos;

  float3 q = fabs(tpos) - scale;
  return fast_length(fmax(q,((float)0))) + fmin(fmax(q.x,fmax(q.y,q.z)),(float)0);
}

struct SceneDist distToScene(__constant uchar8* scene_object_integer_data_buffer,
              __constant float16* scene_object_float_data_buffer,
              uint num_scene_objects,
              float3 point) {
  float min_dist = FLT_MAX;
  uint min_obj = 0;
  for(uint i = 0; i < num_scene_objects; i++){
    float dist;
    switch ( OBJECT_TYPE(scene_object_integer_data_buffer[i]) ) {
      case SPHERE:
        dist = sphereDist(scene_object_float_data_buffer[i], point);
        break;
      case FLOORPLANE:
        dist = floorplaneDist(scene_object_float_data_buffer[i], point);
        break;
      case CAPSULE:
        dist = capsuleDist(scene_object_float_data_buffer[i], point);
        break;
      case CYLINDER:
        dist = cylinderDist(scene_object_float_data_buffer[i], point);
        break;
      case BOX:
        dist = boxDist(scene_object_float_data_buffer[i], point);
        break;
      default:
        dist = FLT_MAX;
        break;
    }
    if (dist < min_dist) {
      min_dist = dist;
      min_obj = i;
    }
  }
  return (struct SceneDist){min_dist, min_obj};
}

struct ClosePoint getPointAtScene( __constant uchar8* scene_object_integer_data_buffer,
                      __constant float16* scene_object_float_data_buffer,
                      uint num_scene_objects,
                      float3 direction,
                      float3 start) {
  float3 curr_point = start;
  uint iterations = 0;
  uint obj_index = 0;
  float dist_to_scene = MAX_DIST-SMALLEST_DIST;
  while(dist_to_scene < MAX_DIST && dist_to_scene > SMALLEST_DIST && iterations < MAX_ITERATIONS){
    struct SceneDist to_scene = distToScene(scene_object_integer_data_buffer, 
                                      scene_object_float_data_buffer, 
                                      num_scene_objects, 
                                      curr_point);

    dist_to_scene = to_scene.dist;
    obj_index = to_scene.obj_index;

    curr_point = curr_point + direction*dist_to_scene;
    iterations++;
  }
  bool out_of_bounds = dist_to_scene >= MAX_DIST || iterations >= MAX_ITERATIONS;
  return (struct ClosePoint){curr_point, iterations, obj_index, out_of_bounds};
}

float3 getNormal(__constant uchar8* scene_object_integer_data_buffer,
                __constant float16* scene_object_float_data_buffer,
                uint num_scene_objects,
                float3 point) {
  
  float dist = distToScene(scene_object_integer_data_buffer,
                          scene_object_float_data_buffer,
                          num_scene_objects,
                          point).dist;

  float3 dx = point - (float3)(SMALLEST_DIST, 0, 0);
  float3 dy = point - (float3)(0, SMALLEST_DIST, 0);
  float3 dz = point - (float3)(0, 0, SMALLEST_DIST);
  
  float normx = dist - distToScene(scene_object_integer_data_buffer,
                                  scene_object_float_data_buffer,
                                  num_scene_objects,
                                  dx).dist;
  
  float normy = dist - distToScene(scene_object_integer_data_buffer,
                                  scene_object_float_data_buffer,
                                  num_scene_objects,
                                  dy).dist;

  float normz = dist - distToScene(scene_object_integer_data_buffer,
                                  scene_object_float_data_buffer,
                                  num_scene_objects,
                                  dz).dist;
                                  
  return fast_normalize((float3)(normx,normy,normz));
}

float getLight (__constant uchar8* scene_object_integer_data_buffer,
                __constant float16* scene_object_float_data_buffer,
                uint num_scene_objects,
                float3 point,
                float3 light){
  float3 scene_normal = getNormal(scene_object_integer_data_buffer,
                                scene_object_float_data_buffer,
                                num_scene_objects,
                                point);

  float3 to_light = fast_normalize(light - point);
  float light_val = dot(to_light, scene_normal);
  
  light_val = clamp(light_val, (float)0 , (float)1);

  struct ClosePoint d = getPointAtScene(scene_object_integer_data_buffer, 
                            scene_object_float_data_buffer, 
                            num_scene_objects, 
                            to_light, 
                            point + scene_normal*NORMAL_EPSILON);

  if (fast_length(point - d.point) < fast_length(point - light)) {
    light_val = light_val*0.2;
  }

  return light_val;
}

float3 getReflection(float3 in, float3 normal) {
  return in - 2*dot(in,normal)*normal;
}

uchar3 rayCastHelper(__constant uchar8* scene_object_integer_data_buffer,
                  __constant float16* scene_object_float_data_buffer,
                  uint num_scene_objects,
                  float3 light_pos,
                  float3 start_point,
                  float3 direction,
                  uint reflect_depth){

  struct ClosePoint d = getPointAtScene(scene_object_integer_data_buffer, 
                              scene_object_float_data_buffer, 
                              num_scene_objects, 
                              direction, 
                              start_point);

  float light = getLight( scene_object_integer_data_buffer, 
                          scene_object_float_data_buffer, 
                          num_scene_objects,
                          d.point,
                          light_pos);
  
  uchar3 color = OBJECT_COLOR(scene_object_integer_data_buffer[d.obj_index]);

  float reflectivity = REFLECTIVITY(scene_object_float_data_buffer[d.obj_index]);

  if(d.out_of_bounds || reflect_depth >= MAX_REFLECTION_DEPTH || reflectivity/(float)reflect_depth < MIN_REFLECTION_CUTOFF){
    return SCALE_UCHAR3_BY_FLOAT(color, light);
  }

  float3 scene_normal = getNormal(scene_object_integer_data_buffer,
                                scene_object_float_data_buffer,
                                num_scene_objects,
                                d.point);

  float3 new_direction = getReflection(direction, scene_normal);

  uchar3 reflect_color = rayCastHelper(scene_object_integer_data_buffer,
                                      scene_object_float_data_buffer,
                                      num_scene_objects,
                                      light_pos,
                                      d.point + scene_normal*NORMAL_EPSILON,
                                      new_direction,
                                      reflect_depth + 1);

  return SCALE_UCHAR3_BY_FLOAT(color, light*(1.-reflectivity)) + SCALE_UCHAR3_BY_FLOAT(reflect_color, reflectivity);
}

__kernel void rayCast(__global uint* pixel_buffer,
                  __constant uchar8* scene_object_integer_data_buffer,
                  __constant float16* scene_object_float_data_buffer,
                  uint num_scene_objects,
                  float8 camera_info,
                  float3 light_pos,
                  uint width, 
                  uint height) {
  ulong wid = (ulong)width;
  uint y = (uint) (get_global_id(0) / wid);
  uint x = (uint) (get_global_id(0) % wid);

  float scale = CAMERA_SCALE(camera_info);
  float zoom = CAMERA_FRAME_DIST(camera_info);
  float3 camera_pos = CAMERA_POS(camera_info);
  float3 camera_rot = CAMERA_ROTATION(camera_info);

  float offx = ((float)x - (float)width/2)/scale;
  float offy = ((float)height/2 - (float)y)/scale;

  float3 direction = (float3)(offx,offy,zoom);
  direction = vecRotate(direction, camera_rot);
  direction = fast_normalize(direction);

  float3 start_point = vecRotateAround(camera_pos + (float3)(offx, offy, 0), camera_rot, camera_pos);

  uchar3 color = rayCastHelper(scene_object_integer_data_buffer,
                                      scene_object_float_data_buffer,
                                      num_scene_objects,
                                      light_pos,
                                      start_point,
                                      direction,
                                      0);
  
  pixel_buffer[get_global_id(0)] = (uint)color.s0 << 16 | (uint)color.s1 << 8 | (uint)color.s2;
}