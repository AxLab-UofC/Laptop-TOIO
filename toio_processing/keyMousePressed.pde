void keyPressed() {
  if (key == CODED) {
    switch (keyCode) {
      //case UP:
      //  cam.rotateX(PI/2);  
      //  break;
      //case DOWN:
      //  cam.rotateX(-PI/2);
      //  break;
      //case LEFT:
      //  cam.rotateY(PI/4); 
      //  break;
      //case RIGHT:
      //  cam.rotateY(-PI/4);  
      //  break;
      case UP:
        playSpeed += 0.5;  
        break;
      case DOWN:
        playSpeed -= 0.5;  
        break;
      
    }
  }
  
  switch(key) {

  case 'f':
    int[][] notes = {{30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 59, 20}, {30, 62, 20}, {30, 60, 20}, {30, 57, 20}};
    midi(0, 1, notes);
    break;
    
  
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
    
   case 'a':
    for (int i = 0; i < pairs.length; i++) {
      pairs[i].multiTarget(0, 0, 0, 30, 0, getCircle((xmax + 45)/ 2, (ymax + 45) / 2, 3 * min(xmax, ymax) / 8, i));
    }
    break;
    
   case 'q':
    motorTarget(0, 0, 0, 0, 50, 0, 200, 200, 90);
    
   case 'w':
    motorTarget(0, 0, 0, 1, 50, 0, 200, 200, 90);
  
  case 'e':
    motorTarget(0, 0, 0, 2, 50, 0, 200, 200, 90);
    
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


    
  case 'c':
    first = new Frame(moveType.BOTTOM, getCircle(0));
    first.execute();
    break;
    
  case 'v':
    visualOn = !visualOn;
    break;
    
  case 'b':
    visualize(getCircle(0));
    break;
    
  case 'n':
    visualizeTop(getCircle(1));
    break;
    
  case 's':
    stop();
    break;
    
  case 'd':
    debugMode = !debugMode;
    break;
    
  case 'l':
    moveLine(6);
    break;
    
  default:
    break;
    
  }
}

Frame first;

void mousePressed() {
}

void mouseReleased() {
}
