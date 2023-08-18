//For Threading Space Style
color StringCol = color(0, 255, 255);
color backgroundCol = (180);
color ceilingCol = color(255, 255, 255);
color floorCol = color(255, 255, 255);
int stringWeight = 4;

void drawDisplay(){
    background (backgroundCol);
  noStroke();
  fill(200);
  rectMode(CENTER);
  background(255);


  long now = System.currentTimeMillis();

  //draw the "mats"
  background (backgroundCol);
  noStroke();
  fill(200);
  rectMode(CENTER);

  //draw floor
  pushMatrix();
  fill(floorCol);
  translate(0, 0, -vert);
  rect(0, 0, xmax, ymax);
  popMatrix();

  //draw ceiling
  pushMatrix();
  fill(ceilingCol);
  translate(0, 0, vert);
  rect(0, 0, xmax, ymax);
  popMatrix();


  for (int i = 0; i < nPairs; i++) {
    pairs[i].checkActive(now);
    boolean topActive = pairs[i].t.isActive;
    boolean bottomActive = pairs[i].b.isActive;

    pushMatrix();
    translate(-xmax/2, -ymax/2, 0);
    stroke(200);
    fill(255);
    strokeWeight(1);

    //Draw Top Toio
    pushMatrix();
    if (visualOn) {
      translate(pairsViz[i].t.x, pairsViz[i].t.y, vert - 5);
      rotate(pairsViz[i].t.theta * PI/180);
      //box(12, 12, 7);
      
      drawCylinder(10,12,10);
    } else {
      if (topActive) {
        translate(pairs[i].t.x, ymax - pairs[i].t.y, vert - 5);
        rotate(pairs[i].t.theta * PI/180);
        //box(12, 12, 7);
        drawCylinder(10,12,10);
      }
    }
    popMatrix();



    //Draw Bottom Toio
    pushMatrix();
    if (visualOn) {
      translate(pairsViz[i].b.x, pairsViz[i].b.y, -vert + 5);
      rotate(pairsViz[i].b.theta * PI/180);
      //box(12, 12, 7);
      drawCylinder(10,12,10);
    } else {
      if (bottomActive) {
        translate(pairs[i].b.x, pairs[i].b.y, -vert + 5);
        rotate(pairs[i].b.theta * PI/180);
        //box(12, 12, 7);
        drawCylinder(10,12,10);
      }
    }
    popMatrix();


    //Draw strings
    stroke(StringCol);
    strokeWeight(stringWeight);
    if (visualOn) {
      line(pairsViz[i].t.x, pairsViz[i].t.y, vert, pairsViz[i].b.x, pairsViz[i].b.y, -vert);
    } else {
      if (topActive && bottomActive) {
        line(pairs[i].t.x, ymax - pairs[i].t.y, vert, pairs[i].b.x, pairs[i].b.y, -vert);
      }
    }

    popMatrix();
  }

  
}

//Function to draw Cylinder
void drawCylinder( int sides, float r, float h)
{
  float angle = 360 / sides;
  float halfHeight = h / 2;
  strokeWeight(2);
  stroke(200);
  // draw top of the tube
  beginShape();
  for (int i = 0; i < sides; i++) {
    float x = cos( radians( i * angle ) ) * r;
    float y = sin( radians( i * angle ) ) * r;
    vertex( x, y, -halfHeight);
  }
  endShape(CLOSE);

  // draw bottom of the tube
  beginShape();
  for (int i = 0; i < sides; i++) {
    float x = cos( radians( i * angle ) ) * r;
    float y = sin( radians( i * angle ) ) * r;
    vertex( x, y, halfHeight);
  }
  endShape(CLOSE);

  // draw sides
  beginShape(TRIANGLE_STRIP);
  noStroke();
  for (int i = 0; i < sides + 1; i++) {
    float x = cos( radians( i * angle ) ) * r;
    float y = sin( radians( i * angle ) ) * r;
    vertex( x, y, halfHeight);
    vertex( x, y, -halfHeight);
  }
  endShape(CLOSE);
}
