void keyPressed() {
  
  switch(key) {  
    case '`':
      toios[0].midi(10, 57, 255);
      break;
      
    case '1':
      toios[0].midi(10, 58, 255);
      break;
      
    case '2':
      toios[0].midi(10, 59, 255);
      break;
      
    case '3':
      toios[0].midi(10, 60, 255);
      break;
      
    case '4':
      toios[0].midi(10, 61, 255);
      break;
      
    case '5':
      toios[0].midi(10, 62, 255);
      break;
      
    case '6':
      toios[0].midi(10, 63, 255);
      break;
      
    case '7':
      toios[0].midi(10, 64, 255);
      break;
      
    case '8':
      toios[0].midi(10, 65, 255);
      break;
      
    case '9':
      toios[0].midi(10, 66, 255);
      break;
      
    case '0':
      toios[0].midi(10, 67, 255);
      break;
      
    case '-':
      toios[0].midi(10, 68, 255);
      break;
      
    case 'k':
    toios[0].led(100, 255, 255, 255);
      break;
      
    case 'm':
      int[][] targets = {{200, 200}, {300, 300}};
      toios[0].multiTarget(0, targets);
      break;
      
    case 'n':
      int[][] notes = {{30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 59, 20}, {30, 62, 20}, {30, 60, 20}, {30, 57, 20}};
      toios[0].midi(1, notes);
      break;
    
    case 'l':
      int[][] lights = {{30, 0, 255, 0}, {30, 0, 0, 255}};
      toios[0].led(5, lights);
      
    default:
      break;
  }
}

void mousePressed() {
  //insert code here;
}

void mouseReleased() {
  //insert code here;
}
