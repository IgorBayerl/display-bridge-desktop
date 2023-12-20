Creating a roadmap for developing a proof of concept (POC) for an app that allows an iPad to be used as an external display for a Linux system involves several key tasks. I'll structure the roadmap in stages, with each task having a point value. These points are a rough estimate of complexity and effort (1 being the simplest, 5 being the most complex).

### Roadmap for Linux Side

#### 1. Environment Setup (Total Points: 5)
   - **Task 1.1: Install Necessary Development Tools (2 points)**
     - Description: Install Linux development environment including compilers, debuggers, and libraries needed for app development.
   - **Task 1.2: Set Up Networking Libraries (3 points)**
     - Description: Install and configure libraries for handling networking and data transfer, essential for communicating with the iPad.

#### 2. Display Capture and Encoding (Total Points: 10)
   - **Task 2.1: Implement Screen Capturing (4 points)**
     - Description: Develop functionality to capture the current screen or a portion of it in real-time.
   - **Task 2.2: Implement Video Encoding (4 points)**
     - Description: Encode captured screen data into a suitable format for transmission.
   - **Task 2.3: Optimize Performance (2 points)**
     - Description: Ensure that screen capturing and encoding are efficient and have minimal impact on system performance.

#### 3. Data Transmission (Total Points: 8)
   - **Task 3.1: Establish USB-C Connection (4 points)**
     - Description: Develop a method to establish a USB-C connection between the Linux computer and the iPad.
   - **Task 3.2: Implement Data Transfer Protocol (4 points)**
     - Description: Create a protocol for reliably and efficiently transferring the encoded video data to the iPad.

#### 4. Testing and Refinement (Total Points: 7)
   - **Task 4.1: Conduct Initial Testing (3 points)**
     - Description: Test the basic functionality of screen capture, encoding, and data transfer.
   - **Task 4.2: Performance Tuning (4 points)**
     - Description: Optimize the performance, focusing on reducing latency and increasing transfer speeds.

### Roadmap for iPadOS App

#### 1. Environment Setup (Total Points: 4)
   - **Task 1.1: Set Up iOS Development Environment (2 points)**
     - Description: Install Xcode and other necessary tools for iPadOS app development.
   - **Task 1.2: Familiarize with iPadOS Development (2 points)**
     - Description: Understand iPadOS specifics, such as UI guidelines and hardware capabilities.

#### 2. App Development (Total Points: 12)
   - **Task 2.1: Create Basic App Structure (3 points)**
     - Description: Set up the basic structure of the app, including the main interface and navigation.
   - **Task 2.2: Implement USB-C Connectivity (4 points)**
     - Description: Develop functionality for the app to establish a USB-C connection with the Linux system.
   - **Task 2.3: Implement Video Decoding and Display (5 points)**
     - Description: Implement decoding of the incoming video data and display it in real-time on the iPad.

#### 3. Testing and Refinement (Total Points: 7)
   - **Task 3.1: Conduct Initial Testing (3 points)**
     - Description: Test the basic functionality of the USB-C connection and video display.
   - **Task 3.2: User Interface and Experience Enhancement (4 points)**
     - Description: Refine the user interface and improve the overall user experience.

#### 4. Integration and End-to-End Testing (Total Points: 8)
   - **Task 4.1: Integrate Linux and iPadOS Apps (4 points)**
     - Description: Ensure that the Linux side and the iPadOS app work seamlessly together.
   - **Task 4.2: Conduct End-to-End Testing (4 points)**
     - Description: Perform comprehensive testing to ensure functionality, performance, and stability.

### Conclusion
This roadmap provides a structured approach to developing a POC for your app. The point values can help in prioritizing tasks and estimating effort. Regular reviews and adjustments may be needed as the project progresses. Remember, this is a complex project, and flexibility in the plan will be key to handling unforeseen challenges.