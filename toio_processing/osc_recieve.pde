

void oscEvent(OscMessage msg) {
  if (msg.checkAddrPattern("/position")) {
    //this collects position information 
    int hostId = msg.get(0).intValue();
    int id = msg.get(1).intValue();
    //int matId = msg.get(1).intValue();
    int posx = msg.get(2).intValue();
    int posy = msg.get(3).intValue();

    int degrees = msg.get(4).intValue();
    //println("Host "+ hostId +" id " + id+" "+posx +" " +posy +" "+degrees);

    id = cubesPerHost*hostId + id;

    if (id < cubes.length) {
      cubes[id].count++;
          


          float elapsedTime = System.currentTimeMillis() -  cubes[id].lastUpdate ;
          cubes[id].speedX = 1000.0 * float(cubes[id].x - cubes[id].prex) / elapsedTime;
          cubes[id].speedY = 1000.0 * float(cubes[id].y - cubes[id].prey) / elapsedTime;
          
          
          cubes[id].prex = cubes[id].x;
          cubes[id].prey = cubes[id].y;

          cubes[id].x = posx;
          cubes[id].y = posy;

          cubes[id].deg = degrees;

          cubes[id].lastUpdate = System.currentTimeMillis();

          float sumX = 0, sumY = 0;
          for (int j = 0; j < cubes[id].aveFrameNum - 1; j++) {
            cubes[id].pre_speedX[cubes[id].aveFrameNum -1 - j] = cubes[id].pre_speedX[cubes[id].aveFrameNum -j -2];
            cubes[id].pre_speedY[cubes[id].aveFrameNum -1 - j] = cubes[id].pre_speedY[cubes[id].aveFrameNum -j -2];
            sumX += cubes[id].pre_speedX[cubes[id].aveFrameNum -1 - j];
            sumY += cubes[id].pre_speedY[cubes[id].aveFrameNum -1 - j];
          }


          sumX +=  cubes[id].speedX;
          sumY +=  cubes[id].speedY;
          
          cubes[id].pre_speedX[0] = cubes[id].speedX;
          cubes[id].pre_speedY[0] = cubes[id].speedY;
          
          //println(cubes[id].speedX, cubes[id].speedY);

          cubes[id].ave_speedX = sumX / float(cubes[id].aveFrameNum);
          cubes[id].ave_speedY = sumY / float(cubes[id].aveFrameNum);
          
          //println(cubes[id].ave_speedX, cubes[id].ave_speedY);
      if (cubes[id].isLost == true) {
        cubes[id].isLost = false;
      }
    }
  } else if (msg.checkAddrPattern("/button")) {
    //this collects button information
    int hostId = msg.get(0).intValue();
    int relid = msg.get(1).intValue();
    int id = cubesPerHost*hostId + relid;
    int pressValue = msg.get(2).intValue();
    
    if (pressValue == 0) {
      cubes[id].onFloor = !cubes[id].onFloor;
      
      if (cubes[id].onFloor) {
        led(id, 100, 0, 0, 255);
      } else {
        led(id, 100, 255, 0, 0);
      }
    }
    
    

    
    println("Button pressed for id " + id + ":  " + pressValue);
  } else if (msg.checkAddrPattern("/motion")) {
    //this collects motion sensor information
    int hostId = msg.get(0).intValue();
    int relid = msg.get(1).intValue();
    int id = cubesPerHost*hostId + relid;
    int flatness = msg.get(2).intValue();
    int hit = msg.get(3).intValue();
    int double_tap = msg.get(4).intValue();
    int face_up = msg.get(5).intValue();
    int shake_level = msg.get(6).intValue();
    //println("Motion for id "+id +": " + flatness +", "+ hit+", "+ double_tap+", "+ face_up+", "+ shake_level);
    
    
  } else if (msg.checkAddrPattern("/magnetic")) {
    //this collects magnetic sensor information
    int hostId = msg.get(0).intValue();
    int relid = msg.get(1).intValue();
    int id = cubesPerHost*hostId + relid;
    int state = msg.get(2).intValue();
    int strength = msg.get(3).intValue();
    int forcex = msg.get(4).intValue();
    int forcey = msg.get(5).intValue();
    int forcez = msg.get(6).intValue();
    println("Magnetic for id "+id +": " + state +", "+ strength + ", " + forcex + ", " + forcey + ", " + forcez);
    
    
  }else if (msg.checkAddrPattern("/postureeuler")) {
    //this collects posture sensor information (in eulers)
    int hostId = msg.get(0).intValue();
    int relid = msg.get(1).intValue();
    int id = cubesPerHost*hostId + relid;
    int roll = msg.get(2).intValue();
    int pitch = msg.get(3).intValue();
    int yaw = msg.get(4).intValue();
    println("Posture for id "+id +": " + roll +", "+ pitch+", "+ yaw);
    
    
  } else if (msg.checkAddrPattern("/posturequaternion")) {
    //this collects posture sensor information (in quaternion)
    int hostId = msg.get(0).intValue();
    int relid = msg.get(1).intValue();
    int id = cubesPerHost*hostId + relid;
    int w = msg.get(2).intValue();
    int x = msg.get(3).intValue();
    int y = msg.get(4).intValue();
    int z = msg.get(5).intValue();
    println("Posture for id "+id +": " + w +", "+ x+", "+ y +", " + z);
    
  } else if (msg.checkAddrPattern("/battery")) {
    //this collects battery value information
    int hostId = msg.get(0).intValue();
    int relid = msg.get(1).intValue();
    int id = cubesPerHost*hostId + relid;
    int battery = msg.get(2).intValue();
    cubes[id].battery = battery;
    //println("Battery Level for id " + id + ": " + battery);
  }
  
}
