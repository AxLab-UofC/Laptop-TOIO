void keyPressed() {
  
  switch(key) {
  case 'f':
    try {
      midi(0, 64, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(1, 63, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 64, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(1, 63, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 64, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(1, 63, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(2, 59, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(3, 62, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(4, 60, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(5, 57, 255, 10);
      
    } catch(InterruptedException e) {
      System.out.println("got interrupted!");
    } 
    break;
    
  case '`':
    midi(0, 57, 255, 10);
    break;
    
  case '1':
    midi(1, 58, 255, 10);
    break;
    
  case '2':
    midi(2, 59, 255, 10);
    break;
    
  case '3':
    midi(3, 60, 255, 10);
    break;
    
  case '4':
    midi(4, 61, 255, 10);
    break;
    
  case '5':
    midi(5, 62, 255, 10);
    break;
    
  case '6':
    midi(6, 63, 255, 10);
    break;
    
  case '7':
    midi(7, 64, 255, 10);
    break;
    
  case '8':
    midi(8, 65, 255, 10);
    break;
    
  case '9':
    midi(9, 66, 255, 10);
    break;
    
  case '0':
    midi(10, 67, 255, 10);
    break;
    
  case '-':
    midi(11, 68, 255, 10);
    break;
    
  case 'd':
    chase = false;
    spin = false;
    mouseDrive = false;
    break;
    
  case 'a':
    for (int i=0; i < nCubes; ++i) {
      aimMotorControl(i, 380, 260);
    }
    break;
    
  case 'k':
    light(0, 100, 255, 0, 0);
    break;
    
  case 'm':
    motion(0);
    break;
    
  default:
    break;
    
  }
}

void mousePressed() {
  chase = false;
  spin = false;
  mouseDrive=true;
}

void mouseReleased() {
  mouseDrive=false;
}
