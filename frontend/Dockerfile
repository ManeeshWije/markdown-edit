FROM node:lts

ARG VITE_SERVER_URL

ENV VITE_SERVER_URL=$VITE_SERVER_URL

WORKDIR /app

COPY package.json .

RUN npm install

COPY . .

RUN npm run build

COPY . .

EXPOSE 3000

CMD ["npm", "run", "preview"]
