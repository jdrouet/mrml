FROM node:lts-buster-slim

WORKDIR /code
COPY package.json package-lock.json /code/
RUN npm ci

COPY src /code/src

ENV PORT=3000

ENTRYPOINT [ "npm", "start" ]
