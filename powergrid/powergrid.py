#!/usr/bin/env python3

import json
import os
import sqlite3
import sys

from flask import Flask, render_template, request, jsonify, send_from_directory

app = Flask(__name__)
app.config.from_object(__name__)

app.config.update({
    'DATABASE': os.environ.get("DATABASE", None) or
    os.path.join(app.root_path, 'powergrid.db'),
})


def db_connect():
    db_path = app.config['DATABASE']
    print("DB: %s" % db_path, file=sys.stderr)
    rv = sqlite3.connect(db_path)
    rv.row_factory = sqlite3.Row
    return rv


def db_get():
    from flask import g
    if not hasattr(g, 'the_database'):
        g.the_database = db_connect()
    return g.the_database


@app.teardown_appcontext
def db_close(error):
    from flask import g
    if hasattr(g, 'the_database'):
        g.the_database.close()


def db_setup():
    db = db_get()
    with app.open_resource('schema.sql', mode='r') as f:
        db.cursor().executescript(f.read())
    db.commit()


@app.cli.command('setupdb')
def initdb_command():
    print('[DB] Initializing [%s]' % app.root_path)
    db_setup()


@app.route('/upload', methods=['POST'])
def upload_run():
    db = db_get()
    run_data = request.get_json()
    run_id = run_data['id']
    try:
        db.execute('insert into runs (id, data) values (?, ?)',
                   [run_id, json.dumps(run_data)])
        db.commit()
    except sqlite3.IntegrityError:
        return "Already exists", 409
    return "Created", 201


def display_est_life(est):
    return '%d:%d' % (est // 3600, est % 3600 // 60)


def extract_info(e):
    d = json.loads(e['data'])
    info = d['system-info']
    hw = info['hardware']
    sw = info['software']
    return {
        'id': d['id'],
        'model': hw['version'],
        'gnome': sw['gnome']['version'],
        'test_name': d['test-name'],
        'est_life': display_est_life(d['estimated-life']),
        'est_power': "%5.2f" % (d['power'])
    }


@app.route('/')
def list_runs():
    return app.send_static_file('index.html')


@app.route('/api/runs')
def api_list_runs():
    db = db_get()
    cur = db.execute('select id, data from runs order by id desc')
    data = cur.fetchall()
    info = [extract_info(e) for e in data]
    return jsonify(info)


@app.route('/api/runs/<runid>', methods=['GET'])
def api_runs_single(runid):
    db = db_get()
    cur = db.execute('select id, data from runs where id = ?', [runid])
    row = cur.fetchall()
    data = json.loads(row[0]['data'])
    return jsonify(data)


@app.route('/static/js/<path:path>')
def static_js(path):
    return app.send_static_file('js/' + path)


@app.route('/static/css/<path:path>')
def send_css(path):
    print(path)
    return app.send_static_file('css/' + path)


if __name__ == '__main__':
    app.run()
