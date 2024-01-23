boolean motorTargetVelocity(int id, int x, int y, float vx, float vy) {

  if (cubes[id].isActive) {
    /////previously defined as .aim
    int left = 0;
    int right = 0;
    float angleToTarget = atan2(y-cubes[id].y, x-cubes[id].x);

    float thisAngle = cubes[id].theta*PI/180;
    float diffAngle = thisAngle-angleToTarget;
    if (diffAngle > PI) diffAngle -= TWO_PI;
    if (diffAngle < -PI) diffAngle += TWO_PI;
    

    //if in front, go forward and
    if (abs(diffAngle) > HALF_PI) { //in front
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

    float aimMotSpeed = velIntegrate / 2.0; // translate the speed (pixel/s)  to motor control command /// Maximum is 115 =>
    //println("Vel Intagrate:", velIntegrate, "aimMotSpeed: ", aimMotSpeed);


    //println("diffVAngle = ", degrees(diffVAngle));
    float aa = 0;
    if (lr[0]<0) { //facing back
      aa = -aimMotSpeed;
    } else { //facing front
      aa = aimMotSpeed;
    }


    float dd = cubes[id].distance(x, y)/50.0;
    dd = min(dd, 1);
    if (dd <.15) cubes[id].motor(0, 0); // keep the motor moving


    float left_ = constrain(aa + (lr[0]*dd), -maxMotorSpeed, maxMotorSpeed);
    float right_ = constrain(aa + (lr[1]*dd), -maxMotorSpeed, maxMotorSpeed);
    int duration = (50);

    //println("motor command:", id, left_, right_);

    cubes[id].motor((int)left_, (int)right_, duration);
  }
  return false;
}
