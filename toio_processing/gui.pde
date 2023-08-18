
import controlP5.*;
ControlP5 cp5;

Accordion accordion;

int twistV = 0;
float tepardR_c = 0;
float tepardR_f = 0;

float lineGap = 0;
float spacing = 0;
int xPos = 0;

Slider2D s;


int guiX = 40;
int guiY = 200;

void setupGUI() {

  cp5 = new ControlP5(this);
  
    cp5.addSlider("globalAngleOffsetSpeed")
    .setPosition(guiX, guiY)
    .setSize(200, 30)
    .setRange(-1, 1)
    .setValue(globalAngleOffsetSpeed);
  ;

  cp5.addSlider("t_offsetAngleSpeed")
    .setPosition(guiX, guiY+40)
    .setSize(200, 30)
    .setRange(-1, 1)
    .setValue(t_offsetAngleSpeed);
  ;

  cp5.addSlider("b_offsetAngleSpeed")
    .setPosition(guiX, guiY+80)
    .setSize(200, 30)
    .setRange(-1, 1)
    .setValue(b_offsetAngleSpeed)
    ;

  cp5.addSlider("t_radiusSpeed")
    .setPosition(guiX, guiY+120)
    .setSize(200, 30)
    .setRange(-2, 2)
    .setValue(t_radiusSpeed)
    ;
    
   cp5.addSlider("b_radiusSpeed")
    .setPosition(guiX, guiY+160)
    .setSize(200, 30)
    .setRange(-2, 2)
    .setValue(b_radiusSpeed)
    ;

  //cp5.addSlider("lineGap")
  //  .setPosition(20, 500)
  //  .setSize(200, 30)
  //  .setRange(-380, 380)
  //  .setValue(0)
  //  ;

  //cp5.addSlider("spacing")
  //  .setPosition(20, 550)
  //  .setSize(200, 30)
  //  .setRange(20, 55)
  //  .setValue(30)
  //  ;

  //cp5.addSlider("xPos")
  //  .setPosition(20, 600)
  //  .setSize(200, 30)
  //  .setRange(-300, 300)
  //  .setValue(0)
  //  ;

  // cp5.addSlider("toggle")
  //.setPosition(20, 700)
  //.setSize(200, 30)
  //.setRange(0, 1)
  //.setValue(0)
  //;

  //s = cp5.addSlider2D("xy")
  //  .setPosition(20, 220)
  //  .setSize(200, 200)
  //  .setMinMax(0, 0, 1000, 1000)
  //  .setValue(0, 0)
  //  //.disableCrosshair()
  //  ;

  cp5.setAutoDraw(false);
}
