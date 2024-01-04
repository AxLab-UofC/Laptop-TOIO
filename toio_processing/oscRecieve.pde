import processing.data.JSONArray;
import processing.data.JSONObject;
void oscEvent(OscMessage msg) {
  //Python Commands
  if (msg.checkAddrPattern("/someCommand")) {
      print("received msg from python");
      // Handle Python command here
  }
  if (msg.checkAddrPattern("/test")) {
      String receivedMessage = msg.get(0).stringValue();
      println("Received from Python: " + receivedMessage);
  }
  if (msg.checkAddrPattern("/user_input")) {
      print("python message" + msg);
      String userInput = msg.get(0).stringValue();
      println("Received user input from Python: " + userInput);
  }
    if (msg.checkAddrPattern("/new_positions")) {
    // Parse the new positions and send motor commands to Toio cubes
    String jsonString = msg.get(0).stringValue();
    print(jsonString);
    jsonString = jsonString.trim();
    jsonString = jsonString.substring(1, jsonString.length() - 1); // Remove the leading and trailing square brackets
    jsonString = jsonString.replaceAll("\'", "\""); // Replace single quotes with double quotes for valid JSON
    print(jsonString);
  
    try {
      // Parse the JSON string into a JSONArray
      JSONArray positionsArray = JSONArray.parse(jsonString);
  
      // Iterate over the array of positions
      for (int i = 0; i < positionsArray.size(); i++) {
        JSONObject positionObject = positionsArray.getJSONObject(i);
        int x = positionObject.getInt("x");
        int y = positionObject.getInt("y");
        int theta = positionObject.getInt("theta");
        
        // Assuming cubeId is the index in the array
        motorTarget(i, 0, x, y, theta); // mode is set as 1 by default
      }
    } 
    catch (Exception e) {
      e.printStackTrace();
      println("Failed to parse JSON string");
    }

  }
  
  // You can add more commands if needed
    
  
  if (msg.checkAddrPattern("/position")) {
    //this collects position information 
    int hostId = msg.get(0).intValue();
    int id = msg.get(1).intValue();
    int posx = msg.get(2).intValue();
    int posy = msg.get(3).intValue();
    int postheta = msg.get(4).intValue();
    
    id = cubesPerHost*hostId + id;
    
    cubes[id].positionUpdate(posx, posy, postheta);
  } 
  
  else if (msg.checkAddrPattern("/battery")) {
    //this collects battery value information
    int hostId = msg.get(0).intValue();
    int id = msg.get(1).intValue();
    int battery = msg.get(2).intValue();
    
    id = cubesPerHost * hostId + id;
    
    cubes[id].batteryUpdate(battery);
  }
  
  else if (msg.checkAddrPattern("/motion")) {
    //this collects motion sensor information
    int hostId = msg.get(0).intValue();
    int id = msg.get(1).intValue();
    int flatness = msg.get(2).intValue();
    int hit = msg.get(3).intValue();
    int double_tap = msg.get(4).intValue();
    int face_up = msg.get(5).intValue();
    int shake_level = msg.get(6).intValue();
    
    id = cubesPerHost*hostId + id;
    
    cubes[id].motionUpdate(flatness, hit, double_tap, face_up, shake_level);
  } 
  
  else if (msg.checkAddrPattern("/magnetic")) {
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
  }
  
  else if (msg.checkAddrPattern("/postureeuler")) {
    //this collects posture sensor information (in eulers)
    int hostId = msg.get(0).intValue();
    int id = msg.get(1).intValue();
    int roll = msg.get(2).intValue();
    int pitch = msg.get(3).intValue();
    int yaw = msg.get(4).intValue();
    
    id = cubesPerHost*hostId + id;
    
    cubes[id].postureUpdate(roll, pitch, yaw);
  } 
  
  else if (msg.checkAddrPattern("/posturequaternion")) {
    //this collects posture sensor information (in quaternion)
    int hostId = msg.get(0).intValue();
    int id = msg.get(1).intValue();
    int w = msg.get(2).intValue();
    int x = msg.get(3).intValue();
    int y = msg.get(4).intValue();
    int z = msg.get(5).intValue();
    
    id = cubesPerHost*hostId + id;
    
    cubes[id].postureUpdate(w, x, y, z);
  } 
  
  else if (msg.checkAddrPattern("/button")) {
    //this collects button information
    int hostId = msg.get(0).intValue();
    int relid = msg.get(1).intValue();
    int pressValue = msg.get(2).intValue();
    
    int id = cubesPerHost*hostId + relid;
    
    if (pressValue == 0) {
      cubes[id].buttonUp();
    } else {
      cubes[id].buttonDown();
    } 
  }
  
  else if (msg.checkAddrPattern("/motorresponse")) {
    //this collects button information
    int hostId = msg.get(0).intValue();
    int relid = msg.get(1).intValue();
    int control = msg.get(2).intValue();
    int response = msg.get(3).intValue();
    
    int id = cubesPerHost*hostId + relid;
    
    println(id, ":", control, response);
  }
}