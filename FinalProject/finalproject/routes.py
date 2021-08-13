from flask import render_template, url_for, flash, redirect, request, jsonify, Response
from flask_login import login_user, current_user, logout_user, login_required
from itertools import groupby
from operator import attrgetter
import io 
from matplotlib.backends.backend_agg import FigureCanvasAgg as FigureCanvas
from matplotlib.figure import Figure
from matplotlib.pyplot import xticks
import matplotlib.dates as mdates
import seaborn as sns
from finalproject import app, bcrypt, db
from finalproject.forms import LoginForm, SignUpForm
from finalproject.models import User, Exercise
import finalproject.c_funcs as c_funcs



@app.route("/")
@login_required
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
        login_user(user)
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
    distinct().\
    all()
    return render_template('exercises.html', title="Exercises", exerciseList=exerciseList)



@app.route("/workout")
@login_required
def workout():
    workoutList = Exercise.query.\
    filter_by(userId=current_user.id).\
    order_by(Exercise.date.desc(), Exercise.name.desc(), Exercise.position).\
    all()
    workouts = [list(g) for k, g in groupby(workoutList, attrgetter('date'))]
    workoutList = []
    for workout in workouts:
        exerciseList = [list(g) for k, g in groupby(workout, attrgetter('name'))]
        workoutList.append(exerciseList)
    return render_template('workout.html', title="Workouts", workouts=workoutList)



@app.route("/logout")
@login_required
def logout():
    flash('Successfully logged out ' + current_user.username, 'success')
    logout_user()
    return redirect(url_for('home'))




@app.route("/addWorkout")
@login_required
def addWorkout():
    return render_template('addWorkout.html', title='Add Workout')




@app.route("/saveWorkout", methods=['GET', 'POST'])
def saveWorkout():
    if request.method == 'POST':
        exer = Exercise(userId=current_user.id,name=request.get_json().get('name'), reps=request.get_json().get('reps'), weight=request.get_json().get('weight'), position=request.get_json().get('pos'))
        db.session.add(exer)
        db.session.commit()
        return "Successfully added workout to database"
    return redirect(url_for('workout'))


@app.route("/getSuggestions")
def getSuggestions():
    res = db.session.query(Exercise.name).\
    filter(Exercise.userId==current_user.id).\
    distinct().\
    all()
    suggestions = []
    for (x,) in res:
        suggestions.append(x)
    return jsonify(data=suggestions)



@app.route("/display")
@login_required
def display():
    title = request.args.get('name')
    res = db.session.query(Exercise).\
    filter(Exercise.userId==current_user.id, Exercise.name == title).\
    order_by(Exercise.date.desc()).\
    all()

    days = [list(g) for k, g in groupby(res, attrgetter('date'))]
    weight = []
    pos = []
    reps = []
    stats = {}
    stats['total_days'] = len(days)
    for day in days:
        for exercise in day:
            weight.append(exercise.weight)
            reps.append(exercise.reps)
            pos.append(exercise.position)
    stats['total_reps'] = c_funcs.sum(reps)
    stats['avg_reps'] = c_funcs.avg(reps)
    stats['total_weight'] = c_funcs.total_lifted(weight, reps)
    stats['avg_weight'] = c_funcs.avg(weight)
    stats['min_reps'] = c_funcs.min(reps)
    stats['min_weight'] = c_funcs.min(weight)
    stats['max_reps'] = c_funcs.max(reps)
    stats['max_weight'] = c_funcs.max(weight)
    stats['best_set'] = c_funcs.best_pos(weight, pos)
    stats['consistant_set'] = c_funcs.consistant_pos(weight,pos)
    return render_template('display.html', title=title, days=days, stats=stats)


@app.route("/plot.png")
def plot():
    f = Figure()
    sns.set_style('darkgrid')
    a = f.add_subplot(1,1,1)
    res = db.session.query(Exercise).\
    filter(Exercise.userId==current_user.id, Exercise.name==request.args.get('name'), Exercise.position==1).\
    order_by(Exercise.date).\
    all()
    weight = []
    date = []
    for elem in res:
        weight.append(elem.weight)
        date.append(elem.date)
        print(elem.date)
    a.plot(date, weight)
    a.set_xticklabels(a.get_xticks(), rotation=25)
    a.xaxis.set_major_formatter(mdates.DateFormatter('%Y-%m-%-d'))
    sns.despine(left=True, bottom=True)
    output = io.BytesIO()
    FigureCanvas(f).print_png(output)
    return Response(output.getvalue(), mimetype='image/png')


