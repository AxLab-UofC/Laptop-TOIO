import oscP5.*;
import netP5.*;
import teilchen.*;
import teilchen.behavior.*;
import teilchen.constraint.*;
import teilchen.cubicle.*;
import teilchen.force.*;
import teilchen.integration.*;
import teilchen.util.*;


Physics mPhysics;

Particle mParticle;





//for OSC
OscP5 oscP5;
//where to send the commands to
NetAddress[] server;


//we'll keep the cubes here
Cube[] cubes;

boolean mouseDrive = false;
boolean chase = false;
boolean spin = false;
boolean drop = false;



void settings() {
  size(1000, 1000, P3D);
}


void setup() {


  // for OSC
  // receive messages on port 3333
  oscP5 = new OscP5(this, 3333);

  //send back to the BLE interface
  //we can actually have multiple BLE bridges
  server = new NetAddress[1]; //only one for now
  //send on port 3334
  server[0] = new NetAddress("127.0.0.1", 3334);
  //server[1] = new NetAddress("192.168.0.103", 3334);
  //server[2] = new NetAddress("192.168.200.12", 3334);


  //create cubes
  cubes = new Cube[nCubes];
  for (int i = 0; i< cubes.length; ++i) {
    cubes[i] = new Cube(i, true);
  }

  //do not send TOO MANY PACKETS
  //we'll be updating the cubes every frame, so don't try to go too high
  frameRate(30);

  mPhysics = new Physics();
  /* create a gravitational force */
  Gravity mGravity = new Gravity();
  /* the direction of the gravity is defined by the 'force' vector */
  mGravity.force().set(0, 30);
  /* forces, like gravity or any other force, can be added to the system. they will be automatically applied to
   all particles */
  mPhysics.add(mGravity);
  /* create a particle and add it to the system */
  mParticle = mPhysics.makeParticle();


}

void draw() {
  motion(0);
  background(255);
  stroke(0);
  long now = System.currentTimeMillis();


  
  //draw the "mat"
  fill(255);
  rect(45, 45, 410, 410);






  //draw the cubes
  for (int i = 0; i < cubes.length; ++i) {
    if (cubes[i].isLost==false) {
      pushMatrix();
      translate(cubes[i].x, cubes[i].y);
      rotate(cubes[i].deg * PI/180);
      rect(-10, -10, 20, 20);
      rect(0, -5, 20, 10);
      popMatrix();
    }
  }

  int time = 0;
  // toio drop code start
  if (drop) {



    final float mDeltaTime = 1.0f / frameRate;
    mPhysics.step(mDeltaTime);

    stroke(255, 255, 0);
    fill(255, 0, 0);

    ellipse(mParticle.position().x, mParticle.position().y, 10, 10);



    stroke(204, 102, 0);

    for (int i = 0; i< nCubes; ++i) {

      if (cubes[i].isLost==false && cubes[i].p_isLost == true) {
        mParticle.position().set(cubes[i].x, cubes[i].y);
        mParticle.velocity().set(0, 0);
        mParticle.velocity().mult(10);
      }
      if (cubes[i].isLost==false) {

        aimCubePosVel(cubes[i].id, mParticle.position().x, mParticle.position().y, mParticle.velocity().y, mParticle.velocity().x);
      }
    }

  }

  // toio drop code end




  if (chase) {
    cubes[0].targetx = cubes[0].x;
    cubes[0].targety = cubes[0].y;
    cubes[1].targetx = cubes[0].x;
    cubes[1].targety = cubes[0].y;
  }
  //makes a circle with n cubes
  if (mouseDrive) {
    float mx = (mouseX);
    float my = (mouseY);
    float cx = 45+410/2;
    float cy = 45+410/2;

    float mulr = 180.0;

    float aMouse = atan2( my-cy, mx-cx);
    float r = sqrt ( (mx - cx)*(mx-cx) + (my-cy)*(my-cy));
    r = min(mulr, r);
    for (int i = 0; i< nCubes; ++i) {
      if (cubes[i].isLost==false) {
        float angle = TWO_PI*i/nCubes;
        float na = aMouse+angle;
        float tax = cx + r*cos(na);
        float tay = cy + r*sin(na);
        fill(255, 0, 0);
        ellipse(tax, tay, 10, 10);
        cubes[i].targetx = tax;
        cubes[i].targety = tay;
      }
    }
  }

  if (spin) {
    motorControl(0, -100, 100, 30);
  }

  if (chase || mouseDrive) {
    //do the actual aim
    for (int i = 0; i< nCubes; ++i) {
      if (cubes[i].isLost==false) {
        fill(0, 255, 0);
        ellipse(cubes[i].targetx, cubes[i].targety, 10, 10);
        aimCubeSpeed(i, cubes[i].targetx, cubes[i].targety);
      }
    }
  }


  //did we lost some cubes?
  for (int i=0; i<nCubes; ++i) {
    // 500ms since last update
    cubes[i].p_isLost = cubes[i].isLost;
    if (cubes[i].lastUpdate < now - 1500 && cubes[i].isLost==false) {
      cubes[i].isLost= true;
    }
  }
}




void keyPressed() {
  
  switch(key) {
  case 'f':
    try {
      midi(0, 64, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 63, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 64, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 63, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 64, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 63, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 59, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 62, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 60, 255, 10);
      java.util.concurrent.TimeUnit.MILLISECONDS.sleep(500);
      midi(0, 57, 255, 10);
      
    } catch(InterruptedException e) {
      System.out.println("got interrupted!");
    } 
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
    drop = true;
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
