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
  try {
    basicMotor(1, false, 200, true, 200);
    java.util.concurrent.TimeUnit.MILLISECONDS.sleep(720);
    basicMotor(1, false, 0, true, 0);
    } catch(InterruptedException e) {
      System.out.println("got interrupted!");
    }
    break;
    
  case 'f':
    try {
      midi(0, 10, 64, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 10, 63, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 10, 64, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 10, 63, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 10, 64, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 10, 63, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 10, 59, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 10, 62, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 10, 60, 255);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 10, 57, 255);
      
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
    midi(0, 57, 255, 10);
    break;
    
  case '1':
    midi(0, 58, 255, 10);
    break;
    
  case '2':
    midi(0, 59, 255, 10);
    break;
    
  case '3':
    midi(0, 60, 255, 10);
    break;
    
  case '4':
    midi(0, 61, 255, 10);
    break;
    
  case '5':
    midi(0, 62, 255, 10);
    break;
    
  case '6':
    midi(0, 63, 255, 10);
    break;
    
  case '7':
    midi(0, 64, 255, 10);
    break;
    
  case '8':
    midi(0, 65, 255, 10);
    break;
    
  case '9':
    midi(0, 66, 255, 10);
    break;
    
  case '0':
    midi(0, 67, 255, 10);
    break;
    
  case '-':
    midi(0, 68, 255, 10);
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
    led(0, 100, 255, 0, 0);
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
