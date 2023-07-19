void moveTargets(int[][] spots) {
   for (int i = 0; i < spots.length; i++) {
    circle(spots[i][1], spots[i][0], 20);
    motorTarget(i, 0, spots[i][0], spots[i][1], spots[i][2]);
  }
}

void moveTargets(float[][] spots) {
   for (int i = 0; i < spots.length; i++) {
    circle(spots[i][0], spots[i][1], 20);
    motorTarget(i, 0, int(spots[i][0]), int(spots[i][1]), int(spots[i][2]));
  }
}

void moveCircle(int x, int y, int r) {
  int count = cubes.length;
  float angle = 2 * PI/count;
  float spots[][] = new float[count][3];
  
  for (int i = 0; i < count; i++) {
    spots[i][0] = x + r*cos(angle*i);
    spots[i][1] = y + r*sin(angle*i);
    spots[i][2] = (360 * i * angle / (2 * PI)) + 90;
  }
  
  moveTargets(spots);
}

void moveCircle(int x, int y, int r, int offset) {
  int count = cubes.length;
  float angle = 2 * PI/count;
  float spots[][] = new float[count][3];
  
  for (int i = 0; i < count; i++) {
    int j = (i + offset) % count;
    spots[i][0] = x + r*cos(angle*j);
    spots[i][1] = y + r*sin(angle*j);
    spots[i][2] = (360 * j * angle / (2 * PI)) + 90;
  }

  moveTargets(spots);
}

void moveLine(int count) {
  float spots[][] = new float[count + 1][3];
  
  for (int i = 0; i <= count; i++) {
    spots[i][0] = int(xmax / 2);
    spots[i][1] = int(ymax * (((.8 * i)/count) + .1));
    spots[i][2] = 90;
  }
  
  moveTargets(spots);
}

void moveLine(int count, int offset) {
  float spots[][] = new float[count + 1][3];
  
  for (int i = 0; i <= count; i++) {
    int j = (i + offset) % count;
    spots[i][0] = int(xmax / 2);
    spots[i][1] = int(ymax * (((.8 * j)/count) + .1));
    spots[i][2] = 90;
  }
  
  offset = 0;
  moveTargets(spots);
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
      led(i, 100, 255, 0, 0);
    }
    else {
      led(i, 100, 0, 0, 255);
    }
  }
}

void stop() {
   for (int i = 0; i < cubes.length; i++) {
    motorBasic(i, 0, 0);
  }
}
