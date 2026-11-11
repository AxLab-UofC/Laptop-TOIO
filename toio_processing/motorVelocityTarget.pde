int maxMotorSpeed = 115;
int velocityTargetingDuration = 50;

float getAngleDiff(float cubeAngle, float angleToTarget) {
  float angleDiff = cubeAngle - angleToTarget;
  if (angleDiff > PI) return angleDiff - TWO_PI;
  if (angleDiff < -PI) return angleDiff + TWO_PI;
  return angleDiff;
}

int[] getMotorSpeeds(float angleDiff) {
  int negativeModifier = abs(angleDiff) > HALF_PI ? 1 : -1;
  float frac = negativeModifier * cos(angleDiff);

  // the outer wheel is always at full speed, while the inner wheel slows down as the angle error grows, causing sharper turns
  int outerWheelSpeed = negativeModifier * maxMotorSpeed;
  int innerWheelSpeed = negativeModifier * floor(maxMotorSpeed * pow(frac, 2));
  
  // return motor speeds as { left, right }, setting the inner and outer wheel speeds based on the angle difference
  return angleDiff > 0 ? new int[] { innerWheelSpeed, outerWheelSpeed } : new int[] { outerWheelSpeed, innerWheelSpeed };
}

// non-angled velocity targeting to smoothly travel along a path of points
void motorVelocityTarget(int id, int x, int y, float vx, float vy) {
  if (!cubes[id].isActive) return;
  
  // get left and right motor speeds
  float cubeAngle = cubes[id].theta * PI / 180;
  float angleDiff = getAngleDiff(cubeAngle, atan2(y - cubes[id].y, x - cubes[id].x));
  int[] motorSpeeds = getMotorSpeeds(angleDiff);

  float velIntegrate = sqrt(sq(vx) + sq(vy)); // integrate velocity x + y
  float aimMotorSpeed = velIntegrate / 2.0; // translate the speed (pixel/s)  to motor control command /// Maximum is 115 =>
  float aa = motorSpeeds[0] < 0 ? -aimMotorSpeed : aimMotorSpeed;
  float dd = min(cubes[id].distance(x, y) / 50.0, 1);
  if (dd < .15) {
    cubes[id].motor(0, 0); // keep the motor moving
    return;
  }

  // motor towards target
  int leftMotorSpeed = (int) constrain(aa + (motorSpeeds[0] * dd), -maxMotorSpeed, maxMotorSpeed);
  int rightMotorSpeed = (int) constrain(aa + (motorSpeeds[1] * dd), -maxMotorSpeed, maxMotorSpeed);
  cubes[id].motor(leftMotorSpeed, rightMotorSpeed, velocityTargetingDuration);
}

// angled velocity targeting to smoothly travel along a path of points while matching angle
void motorVelocityTarget(int id, int x, int y, float vx, float vy, int theta) {
  if (!cubes[id].isActive) return;
  
  // use non-angled velocity targeting at high target distance or target velocity
  float targetVelocity = sqrt(sq(vx) + sq(vy));
  float targetDistance = cubes[id].distance(x,y);
  if (targetVelocity > 10 || targetDistance > 10) {
    motorVelocityTarget(id, x, y, vx, vy);
    return;
  } 
  
  // rotate if the angle difference between the cube angle and the target angle is greater than ten degrees
  float angleDiff = getAngleDiff(cubes[id].theta * PI / 180, theta * PI / 180);
  float angleDiffDegrees = degrees(angleDiff);
  if (abs(angleDiffDegrees) < 10) return;
  
  // motor towards target while matching angle
  int rotateSpeed = angleDiffDegrees > 10 
      ? (int) constrain(map(angleDiffDegrees, 10, 150, 10, maxMotorSpeed), 10.0, maxMotorSpeed) 
      : (int) constrain(map(angleDiffDegrees, -10, -150, -10, -maxMotorSpeed), -10.0, -maxMotorSpeed);
  cubes[id].motor(rotateSpeed, -rotateSpeed, 10);
}
