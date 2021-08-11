from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_bcrypt import Bcrypt 
from flask_login import LoginManager


app = Flask(__name__)
app.config['SECRET_KEY'] = 'secret'
app.config['SQLALCHEMY_DATABASE_URI'] = 'postgres://mhxropawgyehbz:21e76b4d3cb65882b0be14ba08506421d8911badac2549b406854d5d639a559b@ec2-3-233-100-43.compute-1.amazonaws.com:5432/d3ank5j78iqd1q'


db = SQLAlchemy(app)
bcrypt = Bcrypt()
loginManager = LoginManager(app)
loginManager.login_view = 'login'
loginManager.login_message_category = 'info'



from finalproject import routes