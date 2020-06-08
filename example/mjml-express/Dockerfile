FROM node:lts-buster-slim

WORKDIR /code
COPY package.json /code/package.json
COPY package-lock.json /code/package-lock.json
RUN npm ci

COPY src /code/src

ENV PORT=3000

ENTRYPOINT [ "npm", "start" ]
