import { ReadmeViewer } from '#/components/readme-previewer';
import { Card } from '#/components/ui/card';
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/repository/$id/')({
  component: RouteComponent,
})

const readmeContent = `# Sample README

This is a sample README file for the repository.

## Features

- Feature 1
- Feature 2
- Feature 3

## Installation

To install the project, follow these steps:

1. Clone the repository
2. Install dependencies
3. Run the application

## Usage

Provide instructions on how to use the application.

## Contributing

If you would like to contribute, please fork the repository and submit a pull request.

Name | Email | Contributions | Additions | Deletions
--- | --- | --- | --- | ---
Contributor 1 | contributor1@example.com | 50 | 100 | 20
Contributor 2 | contributor2@example.com | 30 | 50 | 10
Contributor 3 | contributor3@example.com | 20 | 30 | 5

## License

This project is licensed under the MIT License.
`;

function RouteComponent() {
  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-2">
        <Card className="p-4">
          <span className="font-semibold">Recent Activity</span>
        </Card>
        <Card className="p-4">
          <span className="font-semibold">Last commits</span>
        </Card>
      </div>
      <Card className="overflow-hidden">
        <ReadmeViewer content={readmeContent} />
      </Card>
    </div>
  );
}
