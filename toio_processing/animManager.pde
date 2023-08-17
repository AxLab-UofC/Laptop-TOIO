enum moveType {
  TOP, BOTTOM, PAIR, INDEPENDENT
}

class Frame {
  moveStatus status = moveStatus.NONE;
  moveType type;
  int[][] targets;
  int[][][] indieTargets;
  
  Frame(moveType tpe, int[][] spots) {
    type = tpe;
    targets = spots;
  }
  
  Frame(int[][][] spots) {
    type = moveType.INDEPENDENT;
    indieTargets = spots;
  }
  
  void execute() {
    status = moveStatus.INPROGRESS;
    switch(type) {
      case PAIR:
        for (int i = 0; i < targets.length; i++) {
          pairs[i].target(0, targets[i][0], targets[i][1], targets[i][2]);
        }
        break;
      
      case TOP:
        for (int i = 0; i < targets.length; i++) {
          pairs[i].t.target(0, targets[i][0], targets[i][1], targets[i][2]);
        }
        break;
      
      case BOTTOM:
        for (int i = 0; i < targets.length; i++) {
          pairs[i].b.target(0, targets[i][0], targets[i][1], targets[i][2]);
        }
        break;
      
      case INDEPENDENT:
        for (int i = 0; i < indieTargets.length; i++) {
          pairs[i].t.target(0, indieTargets[i][0][0], indieTargets[i][0][1], indieTargets[i][0][2]);
          pairs[i].b.target(0, indieTargets[i][1][0], indieTargets[i][1][1], indieTargets[i][1][2]);
        }
        break;
    }
  }
  
  moveStatus checkStatus() {
    moveStatus tempStatus = moveStatus.COMPLETE;
    for (int i = 0; i < pairs.length; i++) {
      switch (type) {
        case PAIR:
          if (pairs[i].t.status == moveStatus.INPROGRESS || pairs[i].b.status == moveStatus.INPROGRESS)  {
            tempStatus = moveStatus.INPROGRESS;
          } 
          
          if (pairs[i].t.status == moveStatus.ERROR) {
            tempStatus = moveStatus.INPROGRESS;
            motorDuration(pairs[i].t.id, 50, 5);
          } else if (pairs[i].t.status == moveStatus.NONE) {
            tempStatus = moveStatus.INPROGRESS;
            pairs[i].t.target(0, targets[i][0], targets[i][1], targets[i][2]);
          }
          
          if (pairs[i].b.status == moveStatus.ERROR) {
            tempStatus = moveStatus.INPROGRESS;
            motorDuration(pairs[i].b.id, 50, 5);
          } else if (pairs[i].b.status == moveStatus.NONE) {
            tempStatus = moveStatus.INPROGRESS;
            pairs[i].b.target(0, targets[i][0], targets[i][1], targets[i][2]);
          }
          break;
          
        case TOP:
          if (pairs[i].t.status == moveStatus.INPROGRESS) {
            tempStatus = moveStatus.INPROGRESS;
          } else if (pairs[i].t.status == moveStatus.ERROR) {
            tempStatus = moveStatus.INPROGRESS;
            motorDuration(pairs[i].t.id, 50, 5);
            pairs[i].t.status = moveStatus.NONE;
          } else if (pairs[i].t.status == moveStatus.NONE) {
            tempStatus = moveStatus.INPROGRESS;
            pairs[i].t.target(0, targets[i][0], targets[i][1], targets[i][2]);
          }
          break;
        case BOTTOM:
          if (pairs[i].b.status == moveStatus.INPROGRESS) {
            tempStatus = moveStatus.INPROGRESS;
          } else if (pairs[i].b.status == moveStatus.ERROR) {
            tempStatus = moveStatus.INPROGRESS;
            motorDuration(pairs[i].b.id, 50, 5);
            pairs[i].b.status = moveStatus.NONE;
          } else if (pairs[i].b.status == moveStatus.NONE) {
            tempStatus = moveStatus.INPROGRESS;
            pairs[i].b.target(0, targets[i][0], targets[i][1], targets[i][2]);
          }
          break;
        case INDEPENDENT:
          if (pairs[i].t.status == moveStatus.INPROGRESS || pairs[i].b.status == moveStatus.INPROGRESS)  {
            tempStatus = moveStatus.INPROGRESS;
          } 
          
          if (pairs[i].t.status == moveStatus.ERROR) {
            tempStatus = moveStatus.INPROGRESS;
            motorDuration(pairs[i].t.id, 50, 5);
          } else if (pairs[i].t.status == moveStatus.NONE) {
            tempStatus = moveStatus.INPROGRESS;
            pairs[i].t.target(0, targets[i][0], targets[i][1], targets[i][2]);
          }
          
          if (pairs[i].b.status == moveStatus.ERROR) {
            tempStatus = moveStatus.INPROGRESS;
            motorDuration(pairs[i].b.id, 50, 5);
          } else if (pairs[i].b.status == moveStatus.NONE) {
            tempStatus = moveStatus.INPROGRESS;
            pairs[i].b.target(0, targets[i][0], targets[i][1], targets[i][2]);
          }
          break;
      }
    }
    
    return tempStatus;
  }
}
