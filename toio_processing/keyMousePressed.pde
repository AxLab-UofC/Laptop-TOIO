void keyPressed() {
  
  switch(key) {
  case 'r':
    try {
      basicMotor(1, true, 200, false, 200);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(720);
      basicMotor(1, true, 0, true, 0);
      } catch(InterruptedException e) {
        System.out.println("got interrupted!");
      }
      break;
  case 'l':
    moveLine(6);
    break;
    
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
    
  case 'm':
    //motorTarget(1, 0, 700, 286, 90);
    magneticRequest(0);
    break;
  
  //case 'a':
  //  motorAcceleration(0, 50, 5, 15, 0, 0, 0, 100);
  //  break;
    
  case 'b':
     //for (int i = 0; i < cubes.length; i++) {
     //  println("Battery level of cube " + i + ": " + cubes[i].battery);
     //}
     basicMotor(0, true, 20, true, 20);
     break;
  case 's':
    basicMotor(0, true, 0, true, 0);
    break;
    
  case 'p':
    postureRequest(0, true);
    break;
   
   case 'o':
     postureRequest(0, false);
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
    
  case 'd':
    chase = false;
    spin = false;
    mouseDrive = false;
    break;
    
  case 'a':
    for (int i=0; i < nCubes; ++i) {
      //aimMotorControl(i, 380, 260)
    }
    break;
    
  case 'k':
    ledAll(100, 0, 255, 0);
    break;
    
  //case 'm':
  //  motion(0);
  //  break;
  
   case 't':
     motorControl(0, 45, 30, 250);
     motorControl(1, 45, 30, 250);
     break; 
  
  case 'y':
     basicMotor(0, true, 45, true, 30);
     basicMotor(1, true, 45, true, 30);
     break; 

  case 'x':
    pair.target(1, 400, 400, 90);
    break;
    
  case 'z':
    pair.target(1, 200, 200 , 90);
    break;
    
    
  case 'c':
    moveCircle(xmax / 2, ymax / 2, 3 * min(xmax, ymax) / 8, offset);
    offset = (offset + 1) % 12;
    break;
    
    
  default:
    break;
    
  }
}

int offset = 0;

void mousePressed() {
  chase = false;
  spin = false;
  mouseDrive=true;
}

void mouseReleased() {
  mouseDrive=false;
}
