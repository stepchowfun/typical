import runCircularDependency from './circular-dependency';
import runComprehensive from './comprehensive';
import runDegenerate from './degenerate';
import runSchemaEvolution from './schema-evolution';

console.log('Running circular dependency integration test\u2026\n');
runCircularDependency();

console.log('\nRunning comprehensive integration test\u2026\n');
runComprehensive();

console.log('\nRunning degenerate integration test\u2026\n');
runDegenerate();

console.log('\nRunning schema evolution integration test\u2026\n');
runSchemaEvolution();
