void keyPressed() {
  
  switch(key) {  
    case '`':
      cubes[0].midi(10, 57, 255);
      break;
      
    case '1':
      cubes[0].midi(10, 58, 255);
      break;
      
    case '2':
      cubes[0].midi(10, 59, 255);
      break;
      
    case '3':
      cubes[0].midi(10, 60, 255);
      break;
      
    case '4':
      cubes[0].midi(10, 61, 255);
      break;
      
    case '5':
      cubes[0].midi(10, 62, 255);
      break;
      
    case '6':
      cubes[0].midi(10, 63, 255);
      break;
      
    case '7':
      cubes[0].midi(10, 64, 255);
      break;
      
    case '8':
      cubes[0].midi(10, 65, 255);
      break;
      
    case '9':
      cubes[0].midi(10, 66, 255);
      break;
      
    case '0':
      cubes[0].midi(10, 67, 255);
      break;
      
    case '-':
      cubes[0].midi(10, 68, 255);
      break;
      
    case 'k':
    cubes[0].led(100, 255, 255, 255);
      break;
      
    case 'm':
      int[][] targets = {{200, 200}, {300, 300}};
      cubes[0].multiTarget(0, targets);
      break;
      
    case 'n':
      int[][] notes = {{30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 59, 20}, {30, 62, 20}, {30, 60, 20}, {30, 57, 20}};
      cubes[0].midi(1, notes);
      break;
    
    case 'l':
      int[][] lights = {{30, 0, 255, 0}, {30, 0, 0, 255}};
      cubes[0].led(5, lights);
      break;
      
    case 'a':
      cubes[0].motor(50, 50, 5);
      break;
      
      
    case 't':
      cubes[0].target(100, 100, 90);
      break;
      
    default:
      break;
  }
}

void mousePressed() {
  cubes[0].target(mouseX, mouseY, 90);
  //insert code here;
}

void mouseReleased() {
  //insert code here;
}

void buttonDown(int id) {
    println("Button Pressed!");
    
    //insert code here
}

void buttonUp(int id) {
    println("Button Released!");
    
    //insert code here
}

void collision(int id) {
    println("Collision Detected!");
    
    //insert code here
}

void doubleTap(int id) {
    println("Double Tap Detected!");
    
    //insert code here
}
