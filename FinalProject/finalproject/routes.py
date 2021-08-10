from flask import render_template, url_for, flash, redirect, request
from flask_login import login_user, current_user, logout_user, login_required
from itertools import groupby
from operator import attrgetter
from finalproject import app, bcrypt, db
from finalproject.forms import LoginForm, SignUpForm
from finalproject.models import User, Exercise
import finalproject.rust_funcs


@app.route("/")
def home():
    return render_template('index.html')



@app.route("/signup", methods=['GET','POST'])
def signup():
    if current_user.is_authenticated:
        return redirect(url_for('home'))
    form = SignUpForm()
    if form.validate_on_submit():
        user = User(username=form.username.data, email=form.email.data, password=bcrypt.generate_password_hash(form.password.data).decode('utf-8'))
        db.session.add(user)
        db.session.commit()
        flash('Account created for ' + form.username.data, 'success')
        return redirect(url_for('home'))
    return render_template('signup.html', title='Sign Up', form=form)  



@app.route("/login", methods=['GET','POST'])
def login():
    if current_user.is_authenticated:
        return redirect(url_for('home'))
    form = LoginForm()
    if form.validate_on_submit():
        user = User.query.filter_by(email=form.email.data).first()
        if not user:
            flash('Invalid email', 'danger')
            return render_template('login.html', title='Login', form=form)
        elif bcrypt.check_password_hash(user.password, form.password.data):
            login_user(user, remember=form.remember.data)
            nextRoute = request.args.get('next')
            if nextRoute:
                return redirect(nextRoute)
            flash('Welcome ' + user.username, 'success')
            return redirect(url_for('home'))
        else:
            flash('Incorrect password', 'danger')
    return render_template('login.html', title='Login', form=form)



@app.route("/account")
@login_required
def account():
    return render_template('account.html', title="Account")



@app.route("/exercises")
@login_required
def exercises():
    exerciseList = db.session.query(Exercise.name).\
    filter(Exercise.userId==current_user.id).\
    order_by(Exercise.name).\
    distinct()
    return render_template('exercises.html', title="Exercises", exerciseList=exerciseList)



@app.route("/workout")
@login_required
def workout():
    workoutList = db.session.\
    query(Exercise.name, Exercise.reps, Exercise.weight, Exercise.position, Exercise.date).\
    filter(Exercise.userId==current_user.id).\
    order_by(Exercise.date.desc()).\
    all()
    workouts = [list(g) for k, g in groupby(workoutList, attrgetter('date'))]
    workoutList = []
    for workout in workouts:
        exerciseList = [list(g) for k, g in groupby(workout, attrgetter('name'))]
        workoutList.append(exerciseList)
    return render_template('workout.html', title="Workouts", workouts=workoutList)



@app.route("/logout")
def logout():
    flash('Successfully logged out ' + current_user.username, 'success')
    logout_user()
    return redirect(url_for('home'))




@app.route("/linkTest")
def linkTest():
    flash(finalproject.rust_funcs.double(9), 'success')
    return redirect(url_for('home'))
