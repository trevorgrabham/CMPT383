from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_bcrypt import Bcrypt 
from flask_login import LoginManager


app = Flask(__name__)
app.config['SECRET_KEY'] = 'secret'
app.config['SQLALCHEMY_DATABASE_URI'] = 'postgres://piwdilislxhcju:e6aeadfb0b4af656e385660e571c5d4ffd9a6c7354b3eac48277c5c3efc2e4b7@ec2-18-213-219-169.compute-1.amazonaws.com:5432/d8j1el7b5crseb'


db = SQLAlchemy(app)
bcrypt = Bcrypt()
loginManager = LoginManager(app)
loginManager.login_view = 'login'
loginManager.login_message_category = 'info'


from finalproject import routes
