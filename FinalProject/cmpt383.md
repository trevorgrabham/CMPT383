# CMPT383 Final Project  

### What is the overall goal of the project?  

The goal of this project was to create a way to keep track of my workouts. This project
will help make it easier to see any progress that is being made, as well as being able to
see more in depth stats about how I differ between sets for different exercises.

### Which languages did you use, and what parts of the system are implemented in each?  

The languages used are C++ as my "systems" language, and javascript and python as my
"scripting" languages.  

The web server was implemented in python using the Flask framework. The dynamic web
features were implemented using javascript, including AJAX calls to the web server.
The statistical computations were done using C++, for an increase in performance.  

### What methods did you use to communicate between languages?  

For communicating between the web server and my C++ code, I used a foreign function
interface (FFI). Pybind was used to compile my C++ code into a .so file, that was
able to be imported into my python code like a normal module.  

For communicating between the web server and my javascript code, AJAX calls were used.
These were implemented on the client side, by using jQuery. These were handled on the
server side, by using specific routes, meant for processing and handling the AJAX
requests.

### Exactly what steps should be taken to get the project working, after getting your code?   

After getting my code and moving directories to the `FinalProject` directory, we first need
to make sure that we have all required python modules installed. To do this we can run   
`pip install -r requirements.txt`  
This should recursively install any required modules. Once we get all of the python modules
installed, we need to compile our C++ code for whatever system this is being run on.
We can do this fairly easily by moving into the `FinalProject/build` directory, and running   
`cmake .. && make`  
If cmake is not installed, it can be installed by running   
`sudo apt-get -y install cmake`  
The last step after compilation will be to move the newly created `.so` file to the proper
directory. This can be done by running   
`mv ./c_funcs.*.so ../finalproject`   
Finally, we can move back to the parent folder `FinalProject` and start our web server
by running   
`python ./run.py`   
Our web server should now be running on `localhost:5000` and we should be able to open
a browser and see it in action.  

### What features should we be looking for when marking your project?

For the full tour of the website, we can start off by creating an account. We will
originally be loaded into the login page, but we can get to the signup page from a link
at the bottom. Once we have created an account, we are directed to the home page, and
can go to the 'Add Workout' tab at the top of the nav bar to enter any workouts that we
have done. We can then view them in the 'Workouts' tab, or we can see more about our
progress for each individual exercise in the 'Exercises' tab. After getting to the
exercises tab, we can click any of the exercises, and see our progress to date.    

To make this more interesting, I would recommend signing into the test user account that
I have created with the credentials:   
`username: testuser`   
`password: cmpt383`   
After signing into this account, we can see the workouts that were completed in the past 
under the 'Workouts' tab, and we can see more detailed statistics on the exercises done
in the 'Exercises' tab.
