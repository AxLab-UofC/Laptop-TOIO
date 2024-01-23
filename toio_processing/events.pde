void keyPressed() {
  
  switch(key) {  
    case '1':
      cubes[0].motor(115, 115, 5);
      break;
      
    case '2':
      //cubes[0].target(200, 200, 0);
      cubes[0].target(0, 200, 200, 0);
      break;
    
    case '3':
      int[][] targets = {{200, 200}, {200, 300, 90}, {300, 300}, {300, 200, 270}, {200, 200, 180}};
      cubes[0].multiTarget(0, targets);
      break;
    
    case '4':
      cubes[0].led(100, 255, 255, 255);
      break;
    
    case '5':
      int[][] lights = {{30, 0, 255, 0}, {30, 0, 0, 255}};
      cubes[0].led(5, lights);
      break;
    
    case '6':
      cubes[0].sound(2, 255);
      break;
    
    case '7':
      cubes[0].midi(10, 68, 255);
      break;
    
    case '8':
      int[][] notes = {{30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 64, 20}, {30, 63, 20}, {30, 59, 20}, {30, 62, 20}, {30, 60, 20}, {30, 57, 20}};
      cubes[0].midi(1, notes);
      break;
    
    default:
      break;
  }
}

void mousePressed() {
  cubes[0].target(mouseX, mouseY, 0);
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
