// the most basic way to move a cube
boolean aimCube(int id, float tx, float ty) {
  if (cubes[id].distance(tx, ty)<25) return true;
  int[] lr = cubes[id].aim(tx, ty);
  float left = (lr[0]*.5);
  float right = (lr[1]*.5);
  int duration = (0);
  motorControl(id, left, right, duration);
  return false;
}



boolean aimCubeSpeed(int id, float tx, float ty) {
  float dd = cubes[id].distance(tx, ty)/100.0;
  
  //println("dd is:" + dd);
  
  dd = min(dd, 1);
  if (dd <.15) return true;

  int[] lr = cubes[id].aim(tx, ty);
  float left = (lr[0])*dd;
  float right = (lr[1])*dd;
  //println("left: "+ lr[0] + ";" + "right: " + lr[1]);
  int duration = (100);
  motorControl(id, left, right, duration);
  return false;
}


//helper functions to drive the cubes

boolean rotateCube(int id, float ta) {
  float diff = ta-cubes[id].deg;
  if (diff>180) diff-=360;
  if (diff<-180) diff+=360;
  if (abs(diff)<8) return true;
  int dir = 1;
  int strength = int(abs(diff) / 10);
  strength = 1;//
  if (diff<0)dir=-1;
  float left = ( 5*(1*strength)*dir);
  float right = (-5*(1+strength)*dir);
  int duration = 100;
  motorControl(id, left, right, duration);

  //println("rotate false "+diff +" "+ id+" "+ta +" "+cubes[id].deg);
  return false;
}
