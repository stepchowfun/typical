import { readFileSync, writeFileSync, unlinkSync } from 'fs';
import { Types } from '../generated/types';

const filePath = '/tmp/request';

function writeToFile() {
  const request = {
    to: 'typical@example.com',
    subject: 'I love Typical!',
    body: 'It makes serialization easy and safe.',
  };

  const requestSize = Types.SendEmailRequest.size(request);
  const dataView = new DataView(new ArrayBuffer(requestSize));
  Types.SendEmailRequest.serialize(dataView, 0, request);
  writeFileSync(filePath, dataView);
}

function readFromFile() {
  const fileContents = readFileSync(filePath);
  const request = Types.SendEmailRequest.deserialize(
    new DataView(
      fileContents.buffer.slice(
        fileContents.byteOffset,
        fileContents.byteOffset + fileContents.byteLength,
      ),
    ),
    0,
  )[1];

  console.log('to:', request.to);
  console.log('subject:', request.subject);
  console.log('body:', request.body);

  unlinkSync(filePath);
}

writeToFile();
readFromFile();
