#define SMALLEST_DIST (float)0.001
#define NORMAL_EPSILON (float)0.1
#define MAX_ITERATIONS 100
#define MAX_DIST 100

#define SPHERE 0
#define SPHERE_POS(a) (float3)(a.s0,a.s1,a.s2)
#define SPHERE_RADIUS(a) a.s3

#define FLOORPLANE 1
#define FLOORPLANE_POS(a) a.s0

#define CAPSULE 2
#define CAPSULE_POS_1(a) (float3)(a.s0,a.s1,a.s2)
#define CAPSULE_POS_2(a) (float3)(a.s3,a.s4,a.s5)
#define CAPSULE_RADIUS(a) a.s6

#define CYLINDER 3
#define CYLINDER_POS_1(a) (float3)(a.s0,a.s1,a.s2)
#define CYLINDER_POS_2(a) (float3)(a.s3,a.s4,a.s5)
#define CYLINDER_RADIUS(a) a.s6

#define BOX 4
#define BOX_POS(a) (float3)(a.s0,a.s1,a.s2)
#define BOX_SCALING(a) (float3)(a.s3,a.s4,a.s5)
#define BOX_ROTATION(a) (float3)(a.s6,a.s7,a.s8)

struct SceneDist {
  float3 point;
  uint iterations;
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

float3 vecRotate(float3 pos,float3 rotation, float3 around) {
  float cosa = cos(rotation.s0);
  float sina = sin(rotation.s0);

  float cosb = cos(rotation.s1);
  float sinb = sin(rotation.s1);

  float cosc = cos(rotation.s2);
  float sinc = sin(rotation.s2);

  float3 tpos = pos - around;

  float x = dot((float3)(cosc*cosb, -sinc*cosa + sinc*sinb*sina, sinc*sina + cosc*sinb*cosa), tpos);
  float y = dot((float3)(sinc*cosb, cosc*cosa + sinc*sinb*sina, -cosc*sina + sinc*sinb*cosa), tpos);
  float z = dot((float3)(-sinb, cosb*sina, cosb*cosa), tpos);

  return (float3)(x,y,z) + around;
}

float boxDist( float16 box_data, float3 point)
{
  float3 rot = BOX_ROTATION(box_data);
  float3 scale = BOX_SCALING(box_data);
  float3 pos = BOX_POS(box_data);

  float3 tpos = vecRotate(point, rot, pos + scale/2) - pos;

  float3 q = fabs(tpos) - scale;
  return fast_length(fmax(q,((float)0))) + fmin(fmax(q.x,fmax(q.y,q.z)),(float)0);
}

float distToScene(__constant uchar* scene_object_type_buffer,
              __constant float16* scene_object_data_buffer,
              uint num_scene_objects,
              float3 point) {
  float min_dist = FLT_MAX;
  for(uint i = 0; i < num_scene_objects; i++){
    float dist;
    switch (scene_object_type_buffer[i]) {
      case SPHERE:
        dist = sphereDist(scene_object_data_buffer[i], point);
        break;
      case FLOORPLANE:
        dist = floorplaneDist(scene_object_data_buffer[i], point);
        break;
      case CAPSULE:
        dist = capsuleDist(scene_object_data_buffer[i], point);
        break;
      case CYLINDER:
        dist = cylinderDist(scene_object_data_buffer[i], point);
        break;
      case BOX:
        dist = boxDist(scene_object_data_buffer[i], point);
        break;
      default:
        dist = FLT_MAX;
        break;
    }
    if (dist < min_dist) {
      min_dist = dist;
    }
  }
  return min_dist;
}

struct SceneDist getPointAtScene( __constant uchar* scene_object_type_buffer,
                      __constant float16* scene_object_data_buffer,
                      uint num_scene_objects,
                      float3 direction,
                      float3 start) {
  float3 curr_point = start;
  uint iterations = 0;
  float dist_to_scene = MAX_DIST-SMALLEST_DIST;
  while(dist_to_scene < MAX_DIST && dist_to_scene > SMALLEST_DIST && iterations < MAX_ITERATIONS){
    dist_to_scene = distToScene(scene_object_type_buffer, 
                                      scene_object_data_buffer, 
                                      num_scene_objects, 
                                      curr_point);

    curr_point = curr_point + direction*dist_to_scene;

    iterations++;
  }
  return (struct SceneDist){curr_point, iterations};
}

float3 getNormal(__constant uchar* scene_object_type_buffer,
                __constant float16* scene_object_data_buffer,
                uint num_scene_objects,
                float3 point) {
  
  float dist = distToScene(scene_object_type_buffer,
                          scene_object_data_buffer,
                          num_scene_objects,
                          point);

  float3 dx = point - (float3)(SMALLEST_DIST, 0, 0);
  float3 dy = point - (float3)(0, SMALLEST_DIST, 0);
  float3 dz = point - (float3)(0, 0, SMALLEST_DIST);
  
  float normx = dist - distToScene(scene_object_type_buffer,
                                  scene_object_data_buffer,
                                  num_scene_objects,
                                  dx);
  
  float normy = dist - distToScene(scene_object_type_buffer,
                                  scene_object_data_buffer,
                                  num_scene_objects,
                                  dy);

  float normz = dist - distToScene(scene_object_type_buffer,
                                  scene_object_data_buffer,
                                  num_scene_objects,
                                  dz);
                                  
  return fast_normalize((float3)(normx,normy,normz));
}

float getLight (__constant uchar* scene_object_type_buffer,
                __constant float16* scene_object_data_buffer,
                uint num_scene_objects,
                float3 point,
                float3 light){
  float3 scene_normal = getNormal(scene_object_type_buffer,
                                scene_object_data_buffer,
                                num_scene_objects,
                                point);

  float3 to_light = fast_normalize(light - point);
  float light_val = dot(to_light, scene_normal);
  
  light_val = clamp(light_val, (float)0 , (float)1);

  struct SceneDist d = getPointAtScene(scene_object_type_buffer, 
                            scene_object_data_buffer, 
                            num_scene_objects, 
                            to_light, 
                            point + scene_normal*NORMAL_EPSILON);

  if (fast_length(point - d.point) < fast_length(point - light)) {
    light_val = light_val*0.2;
  }

  return light_val;
}

__kernel void rayCast(__global uchar3* pixel_buffer,
                  __constant uchar* scene_object_type_buffer,
                  __constant float16* scene_object_data_buffer,
                  uint num_scene_objects,
                  uint width, 
                  uint height) {
  ulong wid = (ulong)width;
  uint y = (uint) (get_global_id(0) / wid);
  uint x = (uint) (get_global_id(0) % wid);

  float scale = 100;
  float zoom = 2;
  float3 camera_pos = (float3)(0,3,0);
  float3 light_pos = (float3)(0,10,5);

  float offx = ((float)x - (float)width/2)/scale;
  float offy = ((float)height/2 - (float)y)/scale;


  float3 frame_pos = (float3)(offx,offy,zoom);
  float3 direction = fast_normalize(frame_pos);
  float3 start_point = camera_pos + frame_pos;

  struct SceneDist d = getPointAtScene(scene_object_type_buffer, 
                              scene_object_data_buffer, 
                              num_scene_objects, 
                              direction, 
                              start_point);

  float light = getLight( scene_object_type_buffer, 
                          scene_object_data_buffer, 
                          num_scene_objects,
                          d.point,
                          light_pos);
  
  uchar light_val = (uchar)(light*255);
  
  pixel_buffer[get_global_id(0)] = (uchar3)(light_val,light_val,light_val);
}