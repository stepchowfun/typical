import { readFileSync, writeFileSync, unlinkSync } from 'fs';
import { Types } from '../generated/types';

const filePath = '/tmp/message';
const { SendEmailRequest } = Types;

function writeToFile(): void {
  const message = {
    to: 'typical@example.com',
    subject: 'I love Typical!',
    body: 'It makes serialization easy and safe.',
  };

  const arrayBuffer = SendEmailRequest.serialize(message);
  writeFileSync(filePath, Buffer.from(arrayBuffer));
}

function readFromFile(): void {
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

  /* eslint-disable no-console -- Allow logging for this example. */

  console.log('to:', message.to);
  console.log('subject:', message.subject);
  console.log('body:', message.body);

  /* eslint-enable no-console -- Re-enable this rule. */
}

writeToFile();
readFromFile();
unlinkSync(filePath);
