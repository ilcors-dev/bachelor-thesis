# node:16-alpine è l'ambiente di esecuzione del nostro container,
# in precedenza abbiamo parlato di Alpine Linux, un sistema operativo
# molto leggero, questa immagine è appunto basata su di esso
FROM node:16-alpine

# setup della working directory del container
WORKDIR /usr/src/app

# copia dei file package.json e package-lock.json
COPY package*.json ./

# installazione delle dipendenze
RUN npm install

# copia dei file sorgente
COPY . .

# esposizione della porta 8080
EXPOSE 8080

# comando di avvio del container -> node index.js
CMD [ "node", "index.js" ]