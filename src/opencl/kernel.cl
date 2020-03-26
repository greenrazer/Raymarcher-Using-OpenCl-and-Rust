#define SMALLEST_DIST (float)0.01
#define NORMAL_EPSILON (float)0.1
#define MAX_ITERATIONS 100

#define SPHERE 0
#define SPHERE_POS(a) (float3)(a.s0,a.s1,a.s2)
#define SPHERE_RADIUS(a) a.s3

#define FLOORPLANE 1
#define FLOORPLANE_POS(a) a.s0

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
  float dist_to_scene = FLT_MAX;
  while(dist_to_scene > SMALLEST_DIST && iterations < MAX_ITERATIONS){
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
  if (light_val < 0) {
    light_val = 0;
  }

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
  float zoom = 1;
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