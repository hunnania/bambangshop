# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. _In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?_ <br>
    > In the observer pattern, creating a trait is needed if the Observers have many types. In Bambangshop, all the observers act the same way, so there is no need to create a trait.

2. _id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?_ <br>
    > Using DashMap is necessary for efficiency because DashMap allows us to easily retrieve a specific subscriber based on its url without having to iterate through a list of subscribers and would ensure that each url is unique, as it would not allow duplicate keys.

3. _When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?_ <br>
    > Using DashMap is better than implementing Singleton pattern because DashMap ensures that your program can safely access and modify the list of subscribers at the same time without causing any issues, while using Singleton pattern requires more complex and error-prone code.

#### Reflection Publisher-2
1. _In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?_ <br>
    > Separating "Service" and "Repository" from the "Model" in the MVC pattern follows the principle of separating concerns or responsibility, ensuring that each component has a clear and distinct role. The "Model" focuses on representing data and business logic, while the "Service" encapsulates application-specific logic and workflow, and the "Repository" handles data access and storage. This separation enhances modularity, reusability, and testability, making it easier to maintain and scale the application over time.

2. _What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?_ <br>
    > If we only use the Model without splitting it into "Service" and "Repository," each part (Program, Subscriber, Notification) would have to do more work than necessary and more complex. This would make the code harder to understand and maintain.

3. _Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects._ <br>
    > Postman helps us test our current work by providing a user-friendly interface to send requests to our APIs and see the responses. We can create different requests (like GET, POST, PUT, DELETE) and customize them with headers, parameters, and body data to simulate different scenarios. Postman also allows us to save our requests as collections for easy access and sharing with the team. It helps us validate that our APIs are working as expected and debug any issues by inspecting the responses.

#### Reflection Publisher-3
1. _Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?_ <br>
    > In this tutorial, we use the push model where the publisher will push the Notification to Subscriber

2. _What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case?_ <br>
    > The pull model has observers retrieve information from the subject when notified, reducing coupling but potentially requiring more effort from observers to fetch the data they need.

3. _Explain what will happen to the program if we decide to not use multi-threading in the notification process._ <br>
    > If we decide not to use multi-threading in the notification process, the program will handle notifications sequentially, one after the other. This means that when a notification needs to be sent, the program will wait for the notification process to complete before moving on to the next notification. While this approach is simpler to implement, it can lead to delays in sending notifications, especially if there are many notifications to send or if sending a notification takes a long time. This could result in a poorer user experience, as notifications may not be delivered promptly.