void keyPressed() {
  
  switch(key) {  
    case '`':
      midi(0, 10, 57, 255);
      break;
      
    case '1':
      midi(0, 10, 58, 255);
      break;
      
    case '2':
      midi(0, 10, 59, 255);
      break;
      
    case '3':
      midi(0, 10, 60, 255);
      break;
      
    case '4':
      midi(0, 10, 61, 255);
      break;
      
    case '5':
      midi(0, 10, 62, 255);
      break;
      
    case '6':
      midi(0, 10, 63, 255);
      break;
      
    case '7':
      midi(0, 10, 64, 255);
      break;
      
    case '8':
      midi(0, 10, 65, 255);
      break;
      
    case '9':
      midi(0, 10, 66, 255);
      break;
      
    case '0':
      midi(0, 10, 67, 255);
      break;
      
    case '-':
      midi(0, 10, 68, 255);
      break;
      
    case 'k':
      led(0, 100, 255, 255, 255);
      break;
      
    case 'm':
      int[][] targets = {{200, 200}, {300, 300}};
      //multiTarget(0, 0, 0, 0, 80, 0, targets);
      multiTarget(0, 0, targets);
      break;
      
    case 'n':
    int[][] notes = {{30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 59, 20}, {30, 62, 20}, {30, 60, 20}, {30, 57, 20}};
    midi(0, 1, notes);
    break;
    
    case 'l':
    int[][] lights = {{30, 0, 255, 0}, {30, 0, 0, 255}};
    led(0, 5, lights);
      
    default:
      break;
  }
}

void mousePressed() {
}

void mouseReleased() {
}

void mouseClicked() {
    OscMessage replyMsg = new OscMessage("/test_reply");
    replyMsg.add("Hello from Processing!");
    pythonOsc.send(replyMsg, pythonAddress);
}
