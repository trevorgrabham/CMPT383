from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_bcrypt import Bcrypt 
from flask_login import LoginManager
from cffi import FFI


app = Flask(__name__)
app.config['SECRET_KEY'] = 'secret'
app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///finalProject.db'


db = SQLAlchemy(app)
bcrypt = Bcrypt()
loginManager = LoginManager(app)
loginManager.login_view = 'login'
loginManager.login_message_category = 'info'
ffi = FFI()
ffi.cdef("""
    int double(int);
""")


from finalproject import routes