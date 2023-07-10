# tourists-visit-cities
# Onboarding_Project
TypeDB data model describing tourists visiting cities.


### Questions to explore
1: Find out how many visited a specific city x
2: Find a city that was visited the most
3: Find how many times person x met a friend in some city

### Setup
To get this repository, run the following command inside your git enabled terminal
```bash
$ git clone https://github.com/Teimurazi-Toloraia/tourists-visit-cities.git
```

You will also need typedb server. Download the zip from  https://repo.vaticle.com/#browse/browse:artifact-snapshot:vaticle_typedb%2F07b9dfe04c786888a68f70b6f46dfdad1c9bb2e5%2Ftypedb-server-windows-07b9dfe04c786888a68f70b6f46dfdad1c9bb2e5.zip>.

**Note:: Click on the path to start the download.

Extract the files and run the following command within the "...\typedb-server-windows-07b.." directory
```
$ ./typedb server --server.address=localhost:1729
```

Once the server is running, head over to the root of this repo that you just cloned and run the rust program using the command
```
$ cargo run
```
The program should give you three query options to choose from until you decide to exit:

```
Choose which query you are interested in:
1: How many people visited city x
2: Find a city that was visited the most
3: Find how many times person x met a friend
0: Exit
```

### Example Query

```
Choose which query you are interested in:
1: How many people visited city x
2: Find a city that was visited the most
3: Find how many times person x met a friend
0: Exit
1
Enter the name of the city
Zugdidi
30 people visited city Zugdidi
Choose which query you are interested in:
1: How many people visited city x
2: Find a city that was visited the most
3: Find how many times person x met a friend
0: Exit
2
most visited city is Tbilisi
Choose which query you are interested in:
1: How many people visited city x
2: Find a city that was visited the most
3: Find how many times person x met a friend
0: Exit
3
Enter the first name of the person
Giorgi           
Enter the last name of the person
Tsintsadze
Giorgi Tsintsadze met a friend 30 times
Choose which query you are interested in:
1: How many people visited city x
2: Find a city that was visited the most
3: Find how many times person x met a friend
0: Exit
0
```