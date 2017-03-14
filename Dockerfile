FROM fedora

RUN dnf -y update && dnf clean all
RUN dnf -y install python3-pip && dnf clean all

EXPOSE 5000
ENV FLASK_APP powergrid
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

ADD . /srv
WORKDIR /srv

RUN pip3 install --requirement /srv/requirements.txt
RUN pip3 install .

RUN flask setupdb
CMD ["flask", "run", "--host=0.0.0.0"]
