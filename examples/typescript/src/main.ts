import { readFileSync, writeFileSync, unlinkSync } from 'fs';
import { Types } from '../generated/types';

const filePath = '/tmp/request';

function writeToFile() {
  const request = {
    to: 'typical@example.com',
    subject: 'I love Typical!',
    body: 'It makes serialization easy and safe.',
  };

  const requestAtlas = Types.SendEmailRequest.atlas(request);
  const dataView = new DataView(new ArrayBuffer(requestAtlas.$size));
  Types.SendEmailRequest.serialize(dataView, 0, request, requestAtlas);
  writeFileSync(filePath, dataView);
}

function readFromFile() {
  const fileContents = readFileSync(filePath);
  const request = Types.SendEmailRequest.deserialize(
    new DataView(
      fileContents.buffer,
      fileContents.byteOffset,
      fileContents.byteLength,
    ),
  );

  console.log('to:', request.to);
  console.log('subject:', request.subject);
  console.log('body:', request.body);
}

writeToFile();
readFromFile();
unlinkSync(filePath);
