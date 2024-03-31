# Welcome to Jeopardy-Api
This Jeopardy API is functionally the same as the alternative that was suggested by Springboard, 
but that one was found to be down recently. Hopefully this will provide some students a way to get
unblocked.

This API also uses the same data as the alternative that was suggested by Springboard.

This application is deployed on Heroku at `https://jeopardy-api-08c22fd2e683.herokuapp.com/`. Please read below to find the endpoints used with the application ( there is no endpoint at the root ).


## API Structure
There are two main sections to this API:
1. Categories - Details about a category
2. Category Details - clues, questions, and answers associated with each category ID.

In addition, there are four endpoints:
1. GET - "/api/categories?<count>" - Gets `count` number of categories. Note that if the count is left off, it will return all categories.
2. GET - "/api/categories/<id>" - Gets a specific category.
3. GET - "/api/details" - Gets all category details (questions, answers, etc.).
4. GET - "/api/details/<category_id>" - Gets all questions/clues for a particular category.

**NOTE:** The provided data had category IDs of [2, 3, 4, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18], so please choose accordingly.


## How to query the API via Curl
To get all category data, you can run this curl:
```
curl -v https://jeopardy-api-08c22fd2e683.herokuapp.com/api/categories
```

To get data for a particular amount of categories, you can add the count parameter, like this:
```
curl -v https://jeopardy-api-08c22fd2e683.herokuapp.com/api/categories?count=5
```
This will return the first five categories ( there are fourteen ).


To get data for a particular category, you can run this curl request:
```
curl -v https://jeopardy-api-08c22fd2e683.herokuapp.com/api/categories/11
```
This will provide category details for the category with the ID of 11.


To get all category details (clues, questions, answers, etc.), you can run:
```
curl -v https://jeopardy-api-08c22fd2e683.herokuapp.com/api/details
```


Finally, to get details for a specific category ID, you can run:
```
curl -v https://jeopardy-api-08c22fd2e683.herokuapp.com/api/details/3
```
This will give you the clues, questions, and answers for all questions under ID #3.


## How to query this via JavaScript's Axios Library
```
const baseUrl = "https://jeopardy-api-08c22fd2e683.herokuapp.com/";
axios.get(`${baseUrl}/api/categories`, {
  headers: {
    'Content-Type': 'application/json'
  },
  params: {
    count: 5
  }
})
.then(response => {
  console.log(response.data);
})
.catch(error => {
  console.error('There was an error!', error);
});
```

You can, of course, use JavaScript's `async / await` syntax as well.


## How to query this via JavaScript's Fetch API
```
fetch('https://jeopardy-api-08c22fd2e683.herokuapp.com/api/categories?count=5', {
  headers: {
    'Content-Type': 'application/json'
  }
})
.then(response => response.json())
.then(data => console.log(data))
.catch(error => console.error('Error:', error));
```


## How to run this locally
As this is intended to be a simple app, no database connection is needed. The category data is self contained in two `.json` files, which is read into memory when the application starts up.

To run the backend for this application, you need to:
1. Have Rust installed. If you do not have Rust installed, please see the installation instructions [here](https://www.rust-lang.org/tools/install) for more information.
2. Clone this repository via `git clone git@github.com:JB0925/jeopardy-api.git`.
3. Simply run `cargo run`. This will build the project and create a socket that binds on port 8000, where it listens for and accepts connections. You can then make the same requests as above; the base url would now be `http://127.0.0.1:8000`.


## How to run the tests
At present, this application has roughly ten tests, mostly covering the normal scenarios and a few edge cases that occur when a user makes a bad request. To run those tests, simply run `cargo test` from your command line.


## Dependencies
This project is based on a Rust web framework called `Rocket`. It also uses a few other dependencies. A full list of them can be found below:
```
once_cell = "1.8.0"
rocket = { version = "0.5.0-rc.2", features = ["json"] }
serde = "1.0.152"
serde_json = "1.0.114"
rocket_cors = "0.6.0"
log = "0.4"
env_logger = "0.11.3"

[dependencies.rocket_contrib]
features = ["json"]
```
