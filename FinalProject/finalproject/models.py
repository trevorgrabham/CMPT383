from datetime import datetime
from finalproject import db, loginManager
from flask_login import UserMixin

@loginManager.user_loader 
def load_user(userId):
    return User.query.get(int(userId))

class User(db.Model, UserMixin):
    id = db.Column(db.Integer, primary_key=True)
    username = db.Column(db.String(30), unique=True, nullable=False)
    email = db.Column(db.String(100), unique=True, nullable=False)
    password = db.Column(db.String(60), nullable=False)
    exercises = db.relationship('Exercise', backref='user', lazy=True)

    def __repr__(self):
        return 'User(' + self.username + ', ' + self.email + ')'

class Exercise(db.Model, UserMixin):
    id = db.Column(db.Integer, primary_key=True)
    userId = db.Column(db.Integer, db.ForeignKey('user.id'), nullable=False)
    name = db.Column(db.String(100), nullable=False)
    reps = db.Column(db.Integer, nullable=False)
    weight = db.Column(db.Integer, nullable=False)      # weight = 0 iff bodyweight
    position = db.Column(db.Integer, nullable=False)
    date = db.Column(db.Date, nullable=False, default=datetime.utcnow)

    def __repr__(self):
        if self.weight > 0:
            return 'Exercise(' + self.name + ', ' + str(self.reps) + ', ' + str(self.weight) + ', ' + self.date + ")"
        else:
            return 'Exercise(' + self.name + ', ' + str(self.reps) + ', body weight, ' + self.date + ")"