# The Oasis

The Oasis is written in Rust and operates on a multithreaded TCP server
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


# Install Brew

	/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"	


### Running the server
Invoke in the terminal and then load 127.0.0.1:8080 in a web browser.

    cargo run 

This code will listen at the address 127.0.0.1:8080 for incoming TCP streams.

Keep server running

    cargo watch -x run