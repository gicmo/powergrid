FROM fedora:latest

RUN dnf -y update && dnf clean all
RUN dnf -y install npm bzip2 python3-pip && dnf clean all

EXPOSE 5000
ENV FLASK_APP powergrid
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

ENV DATABASE="/data/powergrid.db"
RUN mkdir /data && chmod a+rwx /data
VOLUME ["/data"]

WORKDIR /srv
ADD requirements.txt /srv/
RUN pip3 install --requirement requirements.txt

ADD package.json /srv/
RUN npm install &&            \
    npm prune &&              \
    npm cache clean &&        \
    rm -rf ~/.npm

ADD . /srv/

RUN npm run build            && \
    rm -rf /srv/build        && \
    rm -rf /srv/config       && \
    rm -rf /srv/node_modules

RUN pip3 install .

CMD ["/srv/run.sh"]
