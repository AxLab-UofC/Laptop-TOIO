
float overallTimeMillis = 5 * 60 * 1000; // 5 mins
float playSpeed = 1;

float globalAngleOffset=0;
float globalAngleOffsetSpeed = 0.1; //0.05; 


float t_offsetAngle = 0;
float t_offsetAngleSpeed = 0.05; //=-0.05;

float b_offsetAngle = 0;
float b_offsetAngleSpeed =  0; //-0.05;


float t_radius = 300;
float t_radiusSpeed = 0.7;

float b_radius = 330;
float b_radiusSpeed = -2;

int lastMillis = 0;


void animCylinder() {
  


  int elapsedTime = millis() - lastMillis;
  lastMillis = millis();

  visualize(getCylinderTwist(xmax/2, ymax/2, t_radius, b_radius, globalAngleOffset, t_offsetAngle, b_offsetAngle));


  float timeScale = playSpeed * float(elapsedTime)/1000;

  globalAngleOffset += globalAngleOffsetSpeed * timeScale;


  t_radius += t_radiusSpeed * timeScale;
  b_radius += b_radiusSpeed * timeScale;
  
  t_offsetAngle += t_offsetAngleSpeed * timeScale;
  b_offsetAngle += b_offsetAngleSpeed * timeScale;


  if (t_radius > 450) {
    t_radiusSpeed = -abs(t_radiusSpeed);
  } else if (t_radius<100) {
    t_radiusSpeed = abs(t_radiusSpeed);
  }

  if (b_radius > 450) {
    b_radiusSpeed = -abs(b_radiusSpeed);
  } else if (b_radius < 100) {
    b_radiusSpeed = abs(b_radiusSpeed);
  }


  if (t_offsetAngle > radians(360)) {
    t_offsetAngleSpeed = -abs(t_offsetAngleSpeed);
  } else if (b_offsetAngle < radians(-360)) {
    t_offsetAngleSpeed = abs(t_offsetAngleSpeed);
  }
  if (b_offsetAngle > radians(360)) {
    b_offsetAngleSpeed = -abs(b_offsetAngleSpeed);
  } else if (b_offsetAngle < radians(-360)) {
    b_offsetAngleSpeed = abs(b_offsetAngleSpeed);
  }
  
  
}


int [][][] getCylinderTwist(int x, int y, float topR, float bottomR, float globalAngle, float topOffsetAngle, float bottomOffsetAngle) {
  float angle = 2 * PI/(pairs.length-1);
  int spots[][][] = new int[nPairs][2][3];


  for (int i = 0; i < pairs.length; i++) {
    float newAngle = angle*i  +globalAngle;
    spots[i][0][0] = int(x + topR*cos(newAngle + topOffsetAngle)); //x
    spots[i][0][1] = int(y + topR*sin(newAngle + topOffsetAngle)); //y
    spots[i][0][2] = int((360 * (newAngle + topOffsetAngle) / (2 * PI)) + 90); //theta

    spots[i][1][0] = int(x + bottomR*cos(newAngle + bottomOffsetAngle)); //x
    spots[i][1][1]= int(y + bottomR*sin(newAngle + bottomOffsetAngle)); //y
    spots[i][1][2] = int((360 * (newAngle + bottomOffsetAngle) / (2 * PI)) + 90); //theta
  }

  return spots;
}
