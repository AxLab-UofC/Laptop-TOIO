void moveTargets(int[][] spots) {
   for (int i = 0; i < spots.length; i++) {
    circle(spots[i][1], spots[i][0], 20);
    motorTarget(i, 0, spots[i][0], spots[i][1], spots[i][2]);
  }
}

void moveTargets(float[][] spots) {
   for (int i = 0; i < spots.length; i++) {
    circle(spots[i][1], spots[i][0], 20);
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
    int j = (i + offset) % cubes.length;
    spots[i][0] = x + r*cos(angle*j);
    spots[i][1] = y + r*sin(angle*j);
    spots[i][2] = (360 * j * angle / (2 * PI)) + 90;
  }

  moveTargets(spots);
}

void midiAll(int duration, int noteID, int volume) {
  for (int i = 0; i < cubes.length; i++) {
    midi(i, duration, noteID, volume);
  }
}
