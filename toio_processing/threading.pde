void moveCircle(int x, int y, int r) {
  int count = 6;
  float angle = 2 * PI/count;
  float spots[][] = new float[count][3];
  
  for (int i = 0; i < count; i++) {
    spots[i][0] = x + r*cos(angle*i);
    spots[i][1] = y + r*sin(angle*i);
    spots[i][2] = (360 * i * angle / (2 * PI)) + 90;
  }
  
  movePairs(spots);
}

void moveCircle(int x, int y, int r, int offset) {
  int count = 6;
  float angle = 2 * PI/count;
  float spots[][] = new float[count][3];
  
  for (int i = 0; i < count; i++) {
    int j = (i + offset) % count;
    float newAngle = angle*j;
    spots[i][0] = x + r*cos(newAngle);
    spots[i][1] = y + r*sin(newAngle);
    spots[i][2] = (360 * newAngle / (2 * PI)) + 90;
  }

  movePairs(spots);
}

int [][] getCircle(int x, int y, int r, int offset) {
  float angle = 2 * PI/28;
  float angleOffset = offset * (2 * PI/pairs.length);
  int spots[][] = new int[29][3];
  
  for (int i = 0; i < 28; i++) {
    float newAngle = angle*i + angleOffset;
    spots[i][0] = int(x + r*cos(newAngle));
    spots[i][1] = int(y + r*sin(newAngle));
    spots[i][2] = int((360 * (newAngle) / (2 * PI)) + 90);
  }
  
  int[] finalSpot = {int(x + r*cos(angleOffset)), int(y + r*sin(angleOffset)), int((360 * (angleOffset) / (2 * PI)) + 90)};
  spots[28] = finalSpot;

  return spots;
}


void moveLine(int count) {
  float spots[][] = new float[count][3];
  
  for (int i = 0; i < count; i++) {
    spots[i][0] = int(xmax / 2);
    spots[i][1] = int(ymax * (((.8 * i)/count) + .1));
    spots[i][2] = 90;
  }
  
  movePairs(spots);
}

void moveLine(int count, int offset) {
  float spots[][] = new float[count][3];
  
  for (int i = 0; i < count; i++) {
    int j = (i + offset) % count;
    spots[i][0] = int(xmax / 2);
    spots[i][1] = int(ymax * (((.8 * j)/count) + .1));
    spots[i][2] = 90;
  }
  
  offset = 0;
  movePairs(spots);
}

void midiAll(int duration, int noteID, int volume) {
  for (int i = 0; i < cubes.length; i++) {
    midi(i, duration, noteID, volume);
  }
}

void ledAll(int duration, int red, int green, int blue) {
  for (int i = 0; i < cubes.length; i++) {
    led(i, duration, red, green, blue);
  }
}

void ledAll() {
  for (int i = 0; i < cubes.length; i++) {
    if (!cubes[i].onFloor) {
      led(i, 0, 0, 0, 255);
    }
    else {
      led(i, 0, 255, 0, 0);
    }
  }
}
