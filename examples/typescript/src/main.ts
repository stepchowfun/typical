import { readFileSync, writeFileSync, unlinkSync } from 'fs';
import { Types } from '../generated/types';

const filePath = '/tmp/message';
const { SendEmailRequest } = Types;

function writeToFile() {
  const message = {
    to: 'typical@example.com',
    subject: 'I love Typical!',
    body: 'It makes serialization easy and safe.',
  };

  const arrayBuffer = SendEmailRequest.serialize(message);
  writeFileSync(filePath, new DataView(arrayBuffer));
}

function readFromFile() {
  const fileContents = readFileSync(filePath);
  const message = SendEmailRequest.deserialize(
    new DataView(
      fileContents.buffer,
      fileContents.byteOffset,
      fileContents.byteLength,
    ),
  );

  if (message instanceof Error) {
    throw message;
  }

  console.log('to:', message.to);
  console.log('subject:', message.subject);
  console.log('body:', message.body);
}

writeToFile();
readFromFile();
unlinkSync(filePath);
