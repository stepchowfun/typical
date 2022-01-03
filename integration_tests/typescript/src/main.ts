import runCircularDependency from './circular-dependency';
import runComprehensive from './comprehensive';
import runDegenerate from './degenerate';
import runSchemaEvolution from './schema-evolution';

// eslint-disable-next-line no-console
console.log('Running circular dependency integration test\u2026\n');
runCircularDependency();

// eslint-disable-next-line no-console
console.log('\nRunning comprehensive integration test\u2026\n');
runComprehensive();

// eslint-disable-next-line no-console
console.log('\nRunning degenerate integration test\u2026\n');
runDegenerate();

// eslint-disable-next-line no-console
console.log('\nRunning schema evolution integration test\u2026\n');
runSchemaEvolution();
