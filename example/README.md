# function-compose  in REST API development using Axum framework
The example showcases how function-compose can be used in realworld projects. The example uses axum framwork, diesel etc to create 3 RESTAPI. It uses dependency injection, function composition using function-compose    
function-compose is used in the example for code modularization and allow/forces developers to write small functions that can compose.

function-compose also showcases how custom syntax can be developed using rust macros



##### Creating database schema
Run the migration-0.1.sql in your postgres instance. Make sure to change the values of database and schema as required

##### Updating .env file
Update the placeholders <pass-here> <database> and <symmetric-key-here> with your own values.


##### Running the example
To run the project, run the command `cargo run` in terminal. This will start axum based server on port 3000





