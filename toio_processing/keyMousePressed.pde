void keyPressed() {
  
  switch(key) {

  case 'f':
    try {
      midiAll(10, 64, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midiAll(10, 63, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midiAll(10, 64, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midiAll(10, 63, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midiAll(10, 64, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midiAll(10, 63, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midiAll(10, 59, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midiAll(10, 62, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midiAll(10, 60, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midiAll(10, 57, 255);
      
    } catch(InterruptedException e) {
      System.out.println("got interrupted!");
    } 
    break;
    
  
  case '`':
    midiAll(57, 255, 10);
    break;
    
  case '1':
    midiAll(58, 255, 10);
    break;
    
  case '2':
    midiAll(59, 255, 10);
    break;
    
  case '3':
    midiAll(60, 255, 10);
    break;
    
  case '4':
    midiAll(61, 255, 10);
    break;
    
  case '5':
    midiAll(62, 255, 10);
    break;
    
  case '6':
    midiAll(63, 255, 10);
    break;
    
  case '7':
    midiAll(64, 255, 10);
    break;
    
  case '8':
    midiAll(65, 255, 10);
    break;
    
  case '9':
    midiAll(66, 255, 10);
    break;
    
  case '0':
    midiAll(67, 255, 10);
    break;
    
  case '-':
    midiAll(68, 255, 10);
    break;
    
    
  case 'k':
    ledAll();
    break;
    
   case 't':
     motorBasic(0, 45, 30);
     motorBasic(1, 45, 30);
     break; 
  
  case 'y':
     motorBasic(0, true, 45, true, 30);
     motorBasic(1, true, 45, true, 30);
     break; 

  case 'x':
    pairs[0].target(1, 400, 400, 90);
    break;
    
  case 'z':
    pairs[0].target(1, 200, 200 , 90);
    break;
    
  case 'c':
    moveCircle(xmax / 2, ymax / 2, 3 * min(xmax, ymax) / 8, offset);
    offset = (offset + 1) % 6;
    break;
    
  case 's':
    stop();
    break;
    
  case 'l':
    moveLine(6, offset);
    break;
    
  case 'p':
    pairs = makePairs();
    break;
    
    
    
  default:
    break;
    
  }
}

int offset = 0;

void mousePressed() {
}

void mouseReleased() {
}
