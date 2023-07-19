int nCubes =  11;
int frameNum = 5;
int cubesPerHost = 145; // each BLE bridge can have up to 4 cubes
int maxMotorSpeed = 115;
int appFrameRate = 50;

int xmax = 949;
int ymax = 898;

Pair[] makePairs() {
  Pair[] pairs = new Pair[5];
  for (int i = 0; i < 5; i++) {
     pairs[i] = new Pair((i * 2), (i * 2) + 1);
     println((i * 2), (i * 2) + 1);
  }
  return pairs;
}

Pair[] pairs;
