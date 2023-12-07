FROM node:18.17-alpine3.18 

ENV HOME=/opt/app
ENV NODE_OPTIONS="--dns-result-order=ipv4first"

WORKDIR $HOME

# Adding system dependencies
RUN apk --no-cache add libpq libaio libstdc++ libc6-compat  musl musl-dev

COPY ./node /opt/app

RUN npm ci

CMD ["sh", "./start.sh"]