# Learning-Rust
### Learning Rust by example.
- Learning ``` fundamentals ``` and ``` basics ``` of Rust by referring to the documentation " ``` Rust by example ``` " <br />
maintained by the rust community @ https://www.rust-lang.org/learn. <br />
- This repository is a collection of ``` Rust ``` code snippets, templates & tests implemented from the doc ``` Rust by Example ```. <br />
- Documentaion link : https://doc.rust-lang.org/rust-by-example. <br />

# Rust docker image setup
### 1. **Build the docker image 'rust' from the docker file rust.Dockerfile.**

```sh
docker build -t rust -f Envs/rust.Dockerfile Envs
```        
*```Building the image may take 5 to 15 mins. This is done only for the first time.```*

### 2. **Run a container called rust from the built image.**

*```  For windows cmd  ```*
```sh      
docker run --rm -d -t --name=rust -v %cd%:/home/mount rust 
```        

*```For Mac terminal OR windows powershell```*
```sh
docker run --rm -d -t --name=rust -v ${PWD}:/home/mount rust
```

*``` For linux terminal ```*
```sh
docker run --rm -d -t --name=rust -v $(pwd):/home/mount rust
```

### 3. **Open a interactive terminal from the running rust container.**

```sh
docker exec -ti rust bash
```

### 4. **Run any files from within the shell.**
        
```sh
ls
cd Arrays
cd two_sum
node two_sum.js
```
### 5. **To stop and remove the running container.**

```sh
docker stop rust
```
