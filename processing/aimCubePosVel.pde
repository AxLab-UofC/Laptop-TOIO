boolean aimCubePosVel(int id, float tx, float ty, float vx, float vy) {
  
  /////previously defined as .aim
  int left = 0;
  int right = 0;
  float angleToTarget = atan2(ty-cubes[id].y, tx-cubes[id].x);
  float thisAngle = cubes[id].deg*PI/180;
  float diffAngle = thisAngle-angleToTarget;
  if (diffAngle > PI) diffAngle -= TWO_PI;
  if (diffAngle < -PI) diffAngle += TWO_PI;

  //if in front, go forward and
  if (abs(diffAngle) < HALF_PI) { //in front
    float frac = cos(diffAngle);

    if (diffAngle > 0) {
      //up-left
      left = floor(maxMotorSpeed*pow(frac, 2));
      right = maxMotorSpeed;
    } else {
      left = maxMotorSpeed;
      right = floor(maxMotorSpeed*pow(frac, 2));
    }
  } else { //face back

    float frac = -cos(diffAngle);
    if (diffAngle > 0) {
      left  = -floor(maxMotorSpeed*pow(frac, 2));
      right =  -maxMotorSpeed;
    } else {
      left  =  -maxMotorSpeed;
      right = -floor(maxMotorSpeed*pow(frac, 2));
    }
  }
  int[] lr = {left, right};
  // code above came from the previous aim function

  float angleToVelocity = atan2(vy, vx);
  float diffVAngle = thisAngle-angleToVelocity;
  if (diffVAngle > PI) diffVAngle -= TWO_PI;
  if (diffVAngle < -PI) diffVAngle += TWO_PI;

  if (diffAngle > 0) {
    diffVAngle = -diffVAngle;
  }



  float velIntegrate = sqrt(sq(vx)+sq(vy)); // integrate velocity x + y

  float veltoioIntegrate = sqrt(sq(cubes[id].ave_speedX)+sq(cubes[id].ave_speedY));
  float aimMotSpeed = velIntegrate / 2.0; // translate the speed (pixel/s)  to motor control command /// Maximum is 115 =>

  //println("diffVAngle = ", degrees(diffVAngle));
  float aa = 0;
  if (lr[0]<0) { //facing back
    aa = -aimMotSpeed;
  } else { //facing front
    aa = aimMotSpeed;
  }


  float dd = cubes[id].distance(tx, ty)/50.0;
  dd = min(dd, 1);
  //if (dd <.10) return true; // keep the motor moving






  float left_ = constrain(aa + (lr[0]*dd), -115, 115);
  float right_ = constrain(aa + (lr[1]*dd), -115, 115);
  int duration = (50);
  

  
  
  motorControl(id, left_, right_, duration);
  
  

  float d = dist(cubes[id].x, cubes[id].y, tx, ty);

  float targetV_a = atan2( vy, vx);

  //println(degrees(targetV_a), cubes[id].deg);

  //println("targetV: ", velIntegrate, "intendedMotor: ", aimMotSpeed, "  actualSpeed: ", veltoioIntegrate, "|  MotorOutput: ", left_, right_, "| distance: ", dd, d);
  return false;
}
