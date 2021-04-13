# The Oasis Project

Despite being a massively multiplayer online simulation gaming platform,
the Oasis immersed it's users into a global network that connected their everyday 
lives in a virtual environment. The goal of this project is not to make virtual 
reality online gaming possible, but to expand on the idea of network hosting 
a cloud server using cost-effective means.

This project consists of 3 or more Raspberry Pi's connected together using a 
router while employing container technology to deploy applications.  

Containers give the developer the ability to package and application with all the working
libraries and dependencies, and ship it all out as one package.  Building complex
applications that require multiple components across a large array of machines
is difficult.  However, there is one tool that can solve the problem of managing
large clusters of data:  Kubernetes.

If you don't know what Kubernetes is there is a huge knowledge base of information
that you can find on the internet to satisfy your curiosity.  In short, Kubernetes
provides a way to scale down and optimize the use of the underlying hardware which
allows applications to be ported across a network to different systems.  

# Tutorial

1. Components: 

	x 3 Raspberry Pi version 4
	
	x 1 router (stand alone / no gateway)
	
	x 3 Solid State micro SD
	
	enclosure optional

2. Assembly:

To begin this project, we started out with 2 Raspberry Pi's that would serve 
as a client and a server module.  The idea is to have a master node and at least
2 sub-nodes to create a cluster network.  

3. Software Installation

First, we needed to load and operating system.  We chose the latest version of 
Raspian and used the Raspberry Pi Imager to write the image to our SD cards.
Next, we created a new OS image in Virtual Box to facilitate the OS setup and
configuration of software components that will involve our container program.
This also helped us on time by being able to clone the image to create other 
sub-nodes for our cluster.  These can be found in our image file in this 
repository.

For this cluster we assigned the master node with a static IP address of 
192.168.0.50/24.  The sub-nodes PiNode1 and PiNode2 are assigned the addressing
scheme following our master nodes IP assignment accordingly(eg; 192.168.0.51, 52, 53...).
We could have used a different IP addressing scheme but this one will do for
now to serve our purpose.

4. Installing the master node

First, we installed k3s *link to description* using the command:

	curl -sfL https://get.k3s.io | sh -

Next, after the command finishes, there will be a single node cluster created 
and running.  To verify that it is indeed running, use the following command:

	sudo kubectl get nodes

You should see a status list similar to this:

	NAME      STATUS  ROLES   AGE    VERSION
	masterPi  Ready   master  2m13s  v1...

Retrieve the master Pi's join token, which will be used to connect to your nodes, with this command:

	sudo cat /var/lib/rancher/k3s/server/node-token

5. Installing the worker nodes

From your master, boot up a worker node and SSH into it. Run this command to install k3s as a worker node and connect it to
the master:

	curl -sfL http://get.k3s.io | K3S_URL=https://192.168.0.50:6443 \
	K3S_TOKEN=join_token_we_copied_earlier sh -

Replace "join_token_we_copied_earlier" with your master's token from the end of step 4.

Repeat step 5 for each worker node

# Converting a Virtual image to .iso image
We didn't use this due to the fact that the most optimal utility to use in this case was docker.  However,
if you want to give it a try, the following 

	sudo apt install systemback 

# Docker Setup

Since were working with a Raspberry Pi, we need to download and install the arm64 version of docker.  

You can follow the instructions from the Docker website to manually download and install the package, however, Docker provides a convenience script that executes the entire install process:

	curl -fsSL https://get.docker.com -o get-docker.sh
	sudo sh get-docker.sh


# Install Brew for Mac

	/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"	

### Setting up the database
Before we start the Oasis web application the database needs to be established.  There is no need to 
create tables and insert the data as Diesel has already written the migrations for us based on the schema
we want to record user profile creation.  Since the the configurations require PostgreSQL you can either 
navigate to https://www.postgresql.org/download/ and find the latest or use a earlier version that you like.  
This database utilizes Object Relational Mapping where Diesel handles all of the interactions of our records.
I used Ubuntu on WSL and switched MacOSX to get this project started.  However, the target OS in the end will be 
Raspian OS for Raspberry Pi so Brew might not be an option.

Below I have the example of using version 12 for PostgreSQL because it is the most stable however, version
13(beta), which is the most up to date, seems to work just fine.

Ubuntu:

	sudo apt-get install postgresql-12

MacOSX:

	brew install postgresql

After that you need to install the dev library for postgres.

Ubuntu:

	sudo apt-get install libpq

MacOSX:

	brew install libpq

Finally, because postgres doesn't initialize upon download, we must manually start the
server each time or you can add it to something like bashrc.  Anyway, start the postgre service
like so:

Ubuntu:

	sudo service postgresql start
	sudo service postgresql stop // to end the session

MacOSX:

	brew services start postgresql
	brew services stop postgresql // to end the session

Now, we need to do one last thing which is establish the username and password so that Diesel
can access the database and make changes.

	psql postgres // This enters into the postgres database

Then once inside the postgres environment, you can set the password:

	postgres=# \PASSWORD postgres
	Enter new password:

#### Install Diesel CLI crate
Since this application is going to be running in a Kubernetes container I will more than likely
have this dependency set up when you start the service.  However, I love redundancy so I am 
including this step for good measure.  

Diesel will let us work with postgres from the commandline because it is a standalone binary built
into Rust.  This will allow us to store user input for profile creation and allow us to keep the user
logged in.  This is an Object Relational Mapped database.

	cargo install diesel_cli --no-default-features --feature postgresql

We can also use MySQL if you want but you will have to perform the install and change the feature to MySQL
for this ORM to work.

Once Diesel is installed, make sure you are in the application directory and run the following command to 
start Diesel's migration service:

	diesel migration run

That's it!

### Running the application server
Invoke in the terminal and then load 127.0.0.1:8080 in a web browser.

    cargo run 


This code will listen at the address 127.0.0.1:8080 for incoming TCP streams.

Keep server running

    cargo watch -x run

There are 4 pages that the user can navigate to (signup, login, index, submission)
The first page the user should be navigating to is the signup page

	127.0.0.1:8080/signup
There, the user can create a profile with their own credentials where they give a username,
email, and password.

Next the user should navigate to the login page where they can enter the credentials they
provided upon signup.

	127.0.0.1:8080/login
The user will stay logged in until they navigate to the logout page where they will be fully
logged out of the system environment.

	127.0.0.1:8080/logout
